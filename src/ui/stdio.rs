use crate::ui::UserInterface;
use anyhow::Result;
use tokio::io::{self, AsyncBufReadExt, BufReader};

pub struct StdioUI {
    stdin: BufReader<io::Stdin>,
}

impl StdioUI {
    pub fn new() -> Self {
        Self {
            stdin: BufReader::new(io::stdin()),
        }
    }
}

impl UserInterface for StdioUI {
    async fn render(&mut self, event: crate::events::ChatEvent) -> Result<()> {
        match event {
            crate::events::ChatEvent::MessageReceived { author, content } => {
                println!("{}: {}", author, content)
            }
            crate::events::ChatEvent::PeerJoined(name) => {
                println!("> {} joined the room. Say hi!", name)
            }
            crate::events::ChatEvent::PeerLeft(name) => println!("> {} left the chat.", name),
            crate::events::ChatEvent::SystemStatus(status) => println!("> {}", status),
            crate::events::ChatEvent::Error(err) => eprintln!("Error: {}", err),
            crate::events::ChatEvent::PeerNameChange { old, new } => {
                println!("> {} changed their name to {}", old, new)
            }
        }
        Ok(())
    }

    async fn get_input(&mut self) -> Result<Option<String>> {
        let mut line = String::new();
        let bytes = self.stdin.read_line(&mut line).await?;
        if bytes == 0 {
            // EOF
            Ok(None)
        } else {
            Ok(Some(line.trim().to_string()))
        }
    }
}
