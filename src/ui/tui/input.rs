use crate::ui::{InputEvent, InputSource, tui::model::TuiModel};
use ratatui::crossterm::event::{self, Event, KeyCode};
use std::sync::Arc;
use tokio::sync::RwLock;
use tui_input::backend::crossterm::EventHandler;

pub struct TuiInput {
    model: Arc<RwLock<TuiModel>>,
}

impl TuiInput {
    pub(super) fn new(model: Arc<RwLock<TuiModel>>) -> Self {
        Self { model }
    }
}

impl InputSource for TuiInput {
    fn get_input(&mut self) -> anyhow::Result<crate::ui::InputEvent> {
        let mut model = self.model.blocking_write();

        match event::read()? {
            Event::Key(key) => match key.code {
                KeyCode::Enter => {
                    let text = model.input.value_and_reset();
                    if !text.trim().is_empty() {
                        return Ok(InputEvent::Text(text));
                    }
                }
                _ => {
                    model.input.handle_event(&Event::Key(key));
                }
            },
            event => {
                model.input.handle_event(&event);
            }
        }
        Ok(InputEvent::Redraw)
    }
}
