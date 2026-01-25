mod chat;
mod cli;
mod command;
mod dice;
mod events;
mod message;
mod secrets;
mod ticket;
mod ui;

use crate::chat::{ChatApp, ChatClient, ChatConfig, ChatRoom};
use crate::ui::{ChatRenderer, InputEvent, InputSource, UserInterface, stdio::StdioUI};
use crate::{cli::Cli, dice::Dice, events::ChatEvent};
use anyhow::Result;
use tokio::sync::{broadcast, mpsc};
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse()?;
    let chat_config = ChatConfig::from_cli(cli)?;

    let (renderer, mut input_source) = StdioUI::init();
    // UI user input sending logic
    let (input_tx, mut input_rx) = mpsc::channel(100);
    std::thread::spawn(move || {
        loop {
            match input_source.get_input() {
                Ok(InputEvent::Text(text)) => {
                    if input_tx.blocking_send(text).is_err() {
                        break;
                    }
                }
                Ok(InputEvent::Quit) => break,
                Err(e) => {
                    eprintln!("Error fetching input: {}", e);
                    break;
                }
            }
        }
    });

    let (event_tx, mut event_rx) = mpsc::channel(100);
    let (shutdown_tx, _) = broadcast::channel(1);

    let mut app = ChatApp::new(renderer, shutdown_tx.clone());
    let mut connect_task = Box::pin(ChatRoom::connect(chat_config, event_tx));
    let mut backend_handle: Option<JoinHandle<()>> = None;

    let mut exit_reason: Option<String> = None;

    loop {
        tokio::select! {
            // connecting to the chat room in parallel while dataflows work
            result = &mut connect_task, if app.client().is_none() => {
                match result {
                    Ok((client, backend, clipboard)) => {
                        app.set_client(client);
                        app.set_clipboard(clipboard);
                        let shutdown_rx = app.subscribe_shutdown();
                        backend_handle = Some(tokio::spawn(backend.subscribe_loop(shutdown_rx)));
                    }
                    Err(e) => {
                        let err_msg = format!("Fatal connection error: {}", e);
                        if let Err(e) = app.ui().render(ChatEvent::Error(err_msg.clone())).await {
                            eprintln!("Failed to render error to UI: {}. Original error: {}.", e, err_msg)
                        };
                        exit_reason = Some(err_msg);
                        break;
                    }
                }
            }

            // receiving UI and Network (client) events
            event_option = event_rx.recv() => {
                match event_option {
                    Some(event) => app.handle_system_event(event).await.unwrap_or_else(|e|
                        eprintln!("System event handler error: {:#}", e)
                    ),
                    None => break, // On graceful shutdown, we wait until this channel closes
                }
            }

            // receiving user input, processing and propagating as system events
            Some(line) = input_rx.recv() => {
                match app.handle_user_input(line).await {
                    Ok(std::ops::ControlFlow::Continue(_)) => {},
                    Ok(std::ops::ControlFlow::Break(_)) => break,
                    Err(e) => {
                        let err_msg = format!("Input error: {}", e);
                        if let Err(ui_err) = app.ui().render(ChatEvent::Error(err_msg)).await {
                            let msg = format!("Critical UI failure: {}", ui_err);
                            exit_reason = Some(msg);
                            break;
                        }
                    }
                }
            }

            // gracfully handling CTRL+C shutdown signal
            _ = tokio::signal::ctrl_c() => {
                if let Err(e) = app.handle_ctrl_c().await {
                    eprintln!("Error during graceful shutdown: {}", e);
                }
                break;
            }
        }
    }

    shutdown_tx.send(()).ok();

    if let Some(handle) = backend_handle
        && let Err(e) = handle.await
    {
        eprintln!("Backend task panicked: {:?}", e);
    }

    if let Some(reason) = exit_reason {
        eprintln!("Application exited with error: {}", reason);
        return Err(anyhow::anyhow!(reason));
    }

    Ok(())
}
