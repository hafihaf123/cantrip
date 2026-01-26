// pub(crate) mod stdio;
pub(crate) mod tui;

use crate::events::ChatEvent;
use anyhow::Result;

pub trait ChatRenderer: Send + Sync + 'static {
    async fn render(&mut self, event: ChatEvent) -> Result<()>;
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
    Text(String),
    // Quit,
    Redraw,
}
