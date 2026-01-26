use std::sync::Mutex;

use ratatui::widgets::ListState;
use tui_input::Input;

use crate::events::ChatEvent;

#[derive(Default)]
pub(super) struct TuiModel {
    pub(super) messages: Vec<String>,
    pub(super) input: Input,
    pub(super) list_state: Mutex<ListState>,
}

impl TuiModel {
    pub fn apply_event(&mut self, event: &ChatEvent) {
        match event {
            ChatEvent::MessageReceived { author, content } => {
                self.messages.push(format!("{author}: {content}"))
            }
            ChatEvent::PeerJoined(name) => self.messages.push(format!("{name} joined the chat.")),
            ChatEvent::PeerLeft(name) => self.messages.push(format!("{name} left the chat.")),
            ChatEvent::PeerNameChange { old, new } => self
                .messages
                .push(format!("{old} changed their name to '{new}'.")),
            ChatEvent::SystemStatus(text) => self.messages.push(format!("> {text}")),
            ChatEvent::DiceRolled {
                result,
                rolls,
                dice,
                author,
            } => self.messages.push(format!(
                "{} rolled {} from {}  {:?}",
                author.as_deref().unwrap_or("You"),
                result,
                dice,
                rolls
            )),
            ChatEvent::Error(err_msg) => self.messages.push(format!("ERROR: {err_msg}")),
            ChatEvent::Redraw => {}
        }
    }
}
