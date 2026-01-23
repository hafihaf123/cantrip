mod chat;
mod cli;
mod command;
mod dice;
mod events;
mod message;
mod secrets;
mod ticket;
mod ui;

use crate::chat::{ChatClient, ChatConfig, ChatRoom};
use crate::cli::Cli;
use crate::command::InputCommand;
use crate::dice::Dice;
use crate::events::{ChatEvent, NetworkEvent, SystemEvent};
use crate::ui::{UserInterface, stdio::StdioUI};
use anyhow::Result;
use tokio::sync::{broadcast, mpsc};
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse()?;
    let chat_config = ChatConfig::from_cli(cli)?;
    let mut ui = StdioUI::new();

    // UI user input sending logic
    let (input_tx, mut input_rx) = mpsc::channel(100);
    std::thread::spawn(move || {
        let mut ui_input = StdioUI::new();
        loop {
            if let Ok(Some(line)) = ui_input.get_input()
                && input_tx.blocking_send(line).is_err()
            {
                break;
            }
        }
    });

    let (event_tx, mut event_rx) = mpsc::channel(100);
    let (shutdown_tx, _) = broadcast::channel(1);
    let mut connect_task = Box::pin(ChatRoom::connect(chat_config, event_tx));
    let mut client_option: Option<ChatClient> = None;
    let mut backend_handle: Option<JoinHandle<()>> = None;

    loop {
        tokio::select! {
            // connecting to the chat room in parallel while dataflows work
            result = &mut connect_task, if client_option.is_none() => {
                match result {
                    Ok((client, backend)) => {
                        client_option = Some(client);
                        let shutdown_rx = shutdown_tx.subscribe();
                        backend_handle = Some(tokio::spawn(backend.subscribe_loop(shutdown_rx)));
                    }
                    Err(e) => {
                        ui.render(ChatEvent::Error(format!("Fatal connection error: {}", e))).await?;
                        break;
                    }
                }
            }

            // receiving UI and Network (client) events
            event_option = event_rx.recv() => {
                match event_option {
                    Some(event) => handle_system_event(event, &client_option, &mut ui).await?,
                    None => break,
                }
            }

            // receiving user input, processing and propagating as system events
            Some(line) = input_rx.recv() => {
                if !handle_user_input(line, &client_option, &mut ui).await? {
                    // user requested shutdown
                    let _ = shutdown_tx.send(());
                }
            }

            // gracfully handling CTRL+C shutdown signal
            _ = tokio::signal::ctrl_c() => {
                ui.render(ChatEvent::SystemStatus(
                    "Detected shutdown sequence. You can also use the '/quit' command to leave".to_string()
                )).await?;
                if let Some(client) = &client_option {
                    client.broadcast_left().await?;
                    // give some time to the bradcast to succeed
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                }
                let _ = shutdown_tx.send(());
            }
        }
    }

    if let Some(handle) = backend_handle {
        let _ = handle.await;
    }

    Ok(())
}

async fn handle_user_input<T: UserInterface>(
    line: String,
    client_option: &Option<ChatClient>,
    ui: &mut T,
) -> Result<bool> {
    if let Some(client) = client_option {
        let command = InputCommand::from(line);
        match command {
            InputCommand::Quit => {
                client.broadcast_left().await?;
                // give some time to the bradcast to succeed
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                return Ok(false);
            }
            InputCommand::Broadcast(message) => client.broadcast_text(message).await?,
            InputCommand::ChangeName(name) => {
                ui.render(ChatEvent::SystemStatus(format!(
                    "Changed name to {}",
                    &name
                )))
                .await?;
                client.broadcast_join(name).await?;
            }
            InputCommand::DiceRoll(dice_str) => match dice_str.parse::<Dice>() {
                Ok(dice) => {
                    let (result, rolls) = dice.roll();
                    ui.render(ChatEvent::DiceRolled {
                        result,
                        rolls: rolls.clone(),
                        dice,
                        author: None,
                    })
                    .await?;
                    client.broadcast_dice_roll(result, dice, rolls).await?;
                }
                Err(e) => ui.render(ChatEvent::Error(e.to_string())).await?,
            },
        }
    } else {
        ui.render(ChatEvent::Error("Wait for connection...".to_string()))
            .await?;
    }
    Ok(true)
}

async fn handle_system_event<T: UserInterface>(
    event: SystemEvent,
    client_option: &Option<ChatClient>,
    ui: &mut T,
) -> Result<()> {
    match event {
        SystemEvent::Ui(ui_event) => ui.render(ui_event).await?,
        SystemEvent::Network(network_event) => match network_event {
            NetworkEvent::BroadcastJoin(name) => {
                if let Some(client) = client_option {
                    client.broadcast_join(name).await?;
                } else {
                    ui.render(ChatEvent::Error("Waiting for connection...".to_string()))
                        .await?;
                }
            }
        },
    }
    Ok(())
}
