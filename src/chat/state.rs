use tui_input::Input;

use crate::{dice::Dice, events::ChatEvent};

#[derive(Clone, PartialEq)]
pub enum MessageType {
    User(String),
    Me,
    System,
    Dice {
        user: String,
        result: u32,
        rolls: Vec<u32>,
        dice: Dice,
    },
}

pub struct LogMessage {
    pub message_type: MessageType,
    pub content: String,
}

#[derive(Default)]
pub struct AppState {
    messages: Vec<LogMessage>,
    input: Input,
    error_popup: Option<String>,
}

impl AppState {
    pub fn messages(&self) -> &[LogMessage] {
        &self.messages
    }

    pub fn input(&self) -> &Input {
        &self.input
    }

    pub fn input_mut(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn error_popup(&self) -> Option<&str> {
        self.error_popup.as_deref()
    }

    fn push_log(&mut self, message_type: MessageType, content: String) {
        self.messages.push(LogMessage {
            message_type,
            content,
        });
    }

    pub fn close_error_popup(&mut self) {
        self.error_popup = None;
    }

    pub fn apply_event(&mut self, event: ChatEvent) {
        match event {
            ChatEvent::MessageReceived { author, content } => {
                self.push_log(MessageType::User(author), content);
            }
            ChatEvent::PeerJoined(name) => {
                self.push_log(MessageType::System, format!("{name} joined the chat."))
            }
            ChatEvent::PeerLeft(name) => {
                self.push_log(MessageType::System, format!("{name} left the chat."))
            }
            ChatEvent::PeerNameChange { old, new } => self.push_log(
                MessageType::System,
                format!("{old} changed their name to '{new}'."),
            ),
            ChatEvent::SystemStatus(text) => self.push_log(MessageType::System, text),
            ChatEvent::DiceRolled {
                result,
                rolls,
                dice,
                author,
            } => self.push_log(
                MessageType::Dice {
                    user: author.unwrap_or_else(|| "You".to_owned()),
                    result,
                    rolls,
                    dice,
                },
                "".to_owned(),
            ),
            ChatEvent::Error(err_msg) => self.error_popup = Some(err_msg),
            ChatEvent::MessageSent(message) => self.push_log(MessageType::Me, message),
        }
    }
}
