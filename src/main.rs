mod chat_backend;
mod chat_client;
mod chatroom;
mod cli;
mod message;
mod secrets;
mod ticket;

use std::io;

use crate::{chatroom::ChatRoom, cli::Cli};
use anyhow::Result;
use tokio::sync::mpsc;

fn input_loop(line_tx: mpsc::Sender<String>) -> Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    loop {
        stdin.read_line(&mut buffer)?;
        line_tx.blocking_send(buffer.trim_ascii_end().to_string())?;
        buffer.clear();
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse()?;

    let (client, backend) = ChatRoom::join(cli).await?;

    let _backend_handle = tokio::spawn(backend.subscribe_loop());

    let (line_tx, mut line_rx) = mpsc::channel(1);
    let _input_handle = std::thread::spawn(move || input_loop(line_tx));

    println!("> type a message and hit enter to broadcast...");
    while let Some(text) = line_rx.recv().await {
        client.broadcast_text(text).await?;
    }

    Ok(())
}
