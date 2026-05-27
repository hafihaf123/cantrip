use crate::ui::{InputEvent, InputSource};
use ratatui::crossterm::event::{self, Event, KeyCode};

pub struct TuiInput {}

impl TuiInput {
    pub(super) fn new() -> Self {
        Self {}
    }
}

impl InputSource for TuiInput {
    fn get_input(&mut self) -> anyhow::Result<InputEvent> {
        let event = event::read()?;

        match event {
            Event::Key(key) => match key.code {
                KeyCode::Enter => Ok(InputEvent::Submit),
                KeyCode::PageUp | KeyCode::Up => Ok(InputEvent::ScrollUp),
                KeyCode::PageDown | KeyCode::Down => Ok(InputEvent::ScrollDown),
                KeyCode::Esc => Ok(InputEvent::Close),
                _ => Ok(InputEvent::Terminal(Event::Key(key))),
            },
            Event::Resize(..) => Ok(InputEvent::Redraw),
            event => Ok(InputEvent::Terminal(event)),
        }
    }
}
