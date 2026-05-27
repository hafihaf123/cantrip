pub(crate) mod stdio;
pub(crate) mod tui;

use crate::chat::AppState;
use anyhow::Result;
use ratatui::crossterm::event::Event as CrosstermEvent;

pub trait ChatRenderer: Send + Sync + 'static {
    async fn draw(&mut self, state: &AppState) -> Result<()>;

    fn handle_ui_event(&mut self, _: &InputEvent) -> bool {
        false
    }
}

pub trait InputSource: Send + Sync + 'static {
    fn get_input(&mut self) -> Result<InputEvent>;
}

pub trait UserInterface {
    type Renderer: ChatRenderer;
    type Input: InputSource;

    fn init() -> Result<(Self::Renderer, Self::Input)>;
}

pub enum InputEvent {
    Submit,
    ScrollUp,
    ScrollDown,
    Terminal(CrosstermEvent),
    Close,
    Redraw,
}
