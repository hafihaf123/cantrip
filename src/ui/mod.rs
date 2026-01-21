pub(crate) mod stdio;
use crate::events::ChatEvent;
use anyhow::Result;

pub trait UserInterface {
    async fn render(&mut self, event: ChatEvent) -> Result<()>;
    async fn get_input(&mut self) -> Result<Option<String>>;
}
