use crate::ui::{ChatRenderer, InputEvent, InputSource, UserInterface};
use anyhow::Result;
use std::io::{self, BufRead};

pub struct StdioUI {
    stdin: io::Stdin,
}

impl StdioUI {
    fn new() -> Self {
        Self { stdin: io::stdin() }
    }
}

impl UserInterface for StdioUI {
    type Renderer = StdioUI;

    type Input = StdioUI;

    fn init() -> Result<(Self::Renderer, Self::Input)> {
        Ok((StdioUI::new(), StdioUI::new()))
    }
}

impl ChatRenderer for StdioUI {
    async fn render(&mut self, event: crate::events::ChatEvent) -> Result<()> {
        match event {
            crate::events::ChatEvent::MessageReceived { author, content } => {
                println!("{}: {}", author, content);
            }
            crate::events::ChatEvent::PeerJoined(name) => {
                println!("> {} joined the room. Say hi!", name);
            }
            crate::events::ChatEvent::PeerLeft(name) => println!("> {} left the chat.", name),
            crate::events::ChatEvent::SystemStatus(status) => println!("> {}", status),
            crate::events::ChatEvent::Error(err) => eprintln!("Error: {}", err),
            crate::events::ChatEvent::PeerNameChange { old, new } => {
                println!("> {} changed their name to {}", old, new);
            }
            crate::events::ChatEvent::DiceRolled {
                result,
                rolls,
                dice,
                author,
            } => println!(
                "{} rolled {} from {} ({:?})",
                author.as_deref().unwrap_or("You"),
                result,
                dice,
                rolls
            ),
            crate::events::ChatEvent::Redraw => {}
        }
        Ok(())
    }
}

impl InputSource for StdioUI {
    fn get_input(&mut self) -> Result<InputEvent> {
        let mut line = String::new();
        let mut handle = self.stdin.lock();
        let bytes = handle.read_line(&mut line)?;

        if bytes == 0 {
            // EOF
            Ok(InputEvent::Quit)
        } else {
            Ok(InputEvent::Text(line.trim().to_string()))
        }
    }
}
