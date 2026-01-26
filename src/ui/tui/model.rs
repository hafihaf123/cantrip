use crate::events::ChatEvent;
use ratatui::widgets::ListState;
use std::sync::Mutex;
use tui_input::Input;

#[derive(Default)]
pub(super) struct TuiModel {
    pub(super) messages: Vec<String>,
    pub(super) input: Input,
    pub(super) list_state: Mutex<ListState>,
}

impl TuiModel {
    fn push_msg(&mut self, msg: String) {
        let state = self.list_state.get_mut().expect("Mutex poisoned");
        let len = self.messages.len();
        let is_at_bottom = state.selected().is_none_or(|s| s >= len.saturating_sub(1));

        self.messages.push(msg);

        if is_at_bottom {
            state.select(Some(len));
        }
    }

    pub fn scroll_up(&mut self) {
        let state = self.list_state.get_mut().expect("Mutex poisoned");
        let select = state.selected().map(|i| i.saturating_sub(1));
        state.select(select);
    }

    pub fn scroll_down(&mut self) {
        let state = self.list_state.get_mut().expect("Mutex poisoned");
        let select = state.selected().map(|i| i.saturating_add(1));
        if select.unwrap_or(0) < self.messages.len() {
            state.select(select);
        }
    }

    pub fn apply_event(&mut self, event: &ChatEvent) {
        match event {
            ChatEvent::MessageReceived { author, content } => {
                self.push_msg(format!("{author}: {content}"))
            }
            ChatEvent::PeerJoined(name) => self.push_msg(format!("{name} joined the chat.")),
            ChatEvent::PeerLeft(name) => self.push_msg(format!("{name} left the chat.")),
            ChatEvent::PeerNameChange { old, new } => self
                .messages
                .push(format!("{old} changed their name to '{new}'.")),
            ChatEvent::SystemStatus(text) => self.push_msg(format!("> {text}")),
            ChatEvent::DiceRolled {
                result,
                rolls,
                dice,
                author,
            } => self.push_msg(format!(
                "{} rolled {} from {}  {:?}",
                author.as_deref().unwrap_or("You"),
                result,
                dice,
                rolls
            )),
            ChatEvent::Error(err_msg) => self.push_msg(format!("ERROR: {err_msg}")),
            ChatEvent::Redraw => {}
            ChatEvent::MessageSent(message) => self.push_msg(format!("You: {message}")),
        }
    }
}
