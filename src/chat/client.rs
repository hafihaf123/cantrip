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

    pub async fn broadcast_text(&self, text: String) -> Result<()> {
        let message = Message::new(
            MessageBody::Text {
                from: self.endpoint.id(),
                text,
            },
            &self.key,
        )?;
        self.sender
            .broadcast(message.to_vec()?.into())
            .await
            .map_err(Into::into)
    }

    pub async fn broadcast_join(&self, name: String) -> Result<()> {
        let message = Message::new(
            MessageBody::Joined {
                from: self.endpoint.id(),
                name,
            },
            &self.key,
        )?;
        self.sender
            .broadcast(message.to_vec()?.into())
            .await
            .map_err(Into::into)
    }
}
