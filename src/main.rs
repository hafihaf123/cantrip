mod chat;
mod cli;
mod events;
mod message;
mod secrets;
mod ticket;
mod ui;

use crate::chat::{ChatConfig, ChatRoom};
use crate::cli::Cli;
use crate::ui::{UserInterface, stdio::StdioUI};
use anyhow::Result;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse()?;
    let chat_config = ChatConfig::from_cli(cli)?;

    let (event_tx, mut event_rx) = mpsc::channel(100);

    let (client, backend) = ChatRoom::connect(chat_config, event_tx.clone()).await?;

    let _backend_handle = tokio::spawn(backend.subscribe_loop());

    let mut ui = StdioUI::new();

    loop {
        tokio::select! {
            Some(event) = event_rx.recv() => {
                ui.render(event).await?;
            }
            input_result = ui.get_input() => {
                match input_result? {
                    Some(input) => client.broadcast_text(input).await?,
                    None => break,
                }
            }
        }
    }

    Ok(())
}
