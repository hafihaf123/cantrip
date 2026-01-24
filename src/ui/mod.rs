pub(crate) mod stdio;
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

    fn init() -> (Self::Renderer, Self::Input);
}

pub enum InputEvent {
    Text(String),
    Quit,
}
