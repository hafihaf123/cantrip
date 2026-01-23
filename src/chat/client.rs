use crate::dice::Dice;
use crate::message::{Message, MessageBody};
use anyhow::Result;
use iroh::Endpoint;
use iroh_gossip::api::GossipSender;

#[derive(Clone)]
pub struct ChatClient {
    sender: GossipSender,
    endpoint: Endpoint,
    key: [u8; 32],
}

impl ChatClient {
    pub fn new(sender: GossipSender, endpoint: Endpoint, key: [u8; 32]) -> Self {
        Self {
            sender,
            endpoint,
            key,
        }
    }

    async fn broadcast(&self, body: MessageBody) -> Result<()> {
        let message = Message::new(body, &self.key)?;
        self.sender
            .broadcast(message.to_vec()?.into())
            .await
            .map_err(Into::into)
    }

    pub async fn broadcast_text(&self, text: String) -> Result<()> {
        let body = MessageBody::Text {
            from: self.endpoint.id(),
            text,
        };
        self.broadcast(body).await
    }

    pub async fn broadcast_join(&self, name: String) -> Result<()> {
        let body = MessageBody::Joined {
            from: self.endpoint.id(),
            name,
        };
        self.broadcast(body).await
    }

    pub async fn broadcast_left(&self) -> Result<()> {
        let body = MessageBody::Left {
            from: self.endpoint.id(),
        };
        self.broadcast(body).await
    }

    pub async fn broadcast_dice_roll(
        &self,
        result: u32,
        dice: Dice,
        rolls: Vec<u32>,
    ) -> Result<()> {
        let body = MessageBody::DiceRoll {
            from: self.endpoint.id(),
            result,
            dice,
            rolls,
        };
        self.broadcast(body).await
    }
}
