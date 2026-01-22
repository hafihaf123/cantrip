mod chat;
mod cli;
mod events;
mod message;
mod secrets;
mod ticket;
mod ui;

use crate::chat::{ChatClient, ChatConfig, ChatRoom};
use crate::cli::Cli;
use crate::events::{ChatEvent, NetworkEvent, SystemEvent};
use crate::ui::{UserInterface, stdio::StdioUI};
use anyhow::Result;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse()?;
    let chat_config = ChatConfig::from_cli(cli)?;
    let mut ui = StdioUI::new();

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
    let mut connect_task = Box::pin(ChatRoom::connect(chat_config, event_tx.clone()));
    let mut client_option: Option<ChatClient> = None;

    loop {
        tokio::select! {
            result = &mut connect_task, if client_option.is_none() => {
                match result {
                    Ok((client, backend)) => {
                        client_option = Some(client);
                        tokio::spawn(backend.subscribe_loop());
                    }
                    Err(e) => {
                        ui.render(ChatEvent::Error(format!("Fatal connection error: {}", e))).await?;
                        break;
                    }
                }
            }

            Some(event) = event_rx.recv() => {
                handle_system_event(event, &client_option, &mut ui).await?;
            }

            Some(line) = input_rx.recv() => {
                if !handle_user_input(line, &client_option, &mut ui).await? {
                    break;
                }
            }
        }
    }

    Ok(())
}

async fn handle_user_input<T: UserInterface>(
    line: String,
    client_option: &Option<ChatClient>,
    ui: &mut T,
) -> Result<bool> {
    if let Some(client) = client_option {
        client.broadcast_text(line).await?;
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
