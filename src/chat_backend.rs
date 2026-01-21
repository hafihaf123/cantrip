use crate::{
    chatroom::ChatCommand,
    events::ChatEvent,
    message::{Message, MessageBody},
};
use anyhow::Result;
use futures_lite::StreamExt;
use iroh::{Endpoint, EndpointId, protocol::Router};
use iroh_gossip::api::{Event, GossipReceiver};
use std::collections::HashMap;
use tokio::sync::mpsc::Sender;

#[derive(Default)]
struct ChatState {
    users: HashMap<EndpointId, String>,
}

impl ChatState {
    fn update_user(&mut self, author: EndpointId, name: String) -> Option<String> {
        self.users.insert(author, name)
    }

    fn resolve_name(&self, author: EndpointId) -> &str {
        self.users
            .get(&author)
            .map(|s| s.as_str())
            .unwrap_or("Unknown")
    }
}

pub struct ChatBackend {
    _endpoint: Endpoint,
    key: [u8; 32],
    state: ChatState,
    router: Router,
    receiver: GossipReceiver,
    outbox: Sender<ChatCommand>,
    ui_event_tx: Sender<ChatEvent>,
}

impl ChatBackend {
    pub fn new(
        _endpoint: Endpoint,
        key: [u8; 32],
        router: Router,
        receiver: GossipReceiver,
        outbox: Sender<ChatCommand>,
        ui_event_tx: Sender<ChatEvent>,
    ) -> Self {
        Self {
            _endpoint,
            key,
            state: ChatState::default(),
            router,
            receiver,
            outbox,
            ui_event_tx,
        }
    }

    pub async fn subscribe_loop(mut self) {
        while let Some(event) = self.receiver.try_next().await.unwrap_or(None) {
            if let Err(e) = self.handle_event(event).await {
                if self
                    .ui_event_tx
                    .send(ChatEvent::Error(format!(
                        "Failed to process event: {:?}",
                        e
                    )))
                    .await
                    .is_err()
                {
                    break;
                };
            }
        }

        if let Err(e) = self.router.shutdown().await {
            self.ui_event_tx
                .send(ChatEvent::Error(format!("Error during shutdown: {:?}", e)))
                .await
                .ok();
        }
    }

    async fn handle_event(&mut self, event: Event) -> Result<()> {
        if let Event::Received(msg) = event {
            let decrypted = Message::from_bytes(&msg.content)?.decrypt(&self.key)?;
            self.handle_message(decrypted).await?;
        }
        Ok(())
    }

    async fn handle_message(&mut self, body: MessageBody) -> Result<()> {
        match body {
            MessageBody::Joined { from, name } => {
                match self.state.update_user(from, name.clone()) {
                    None => {
                        self.outbox.send(ChatCommand::BroadcastJoin).await?;
                        self.ui_event_tx.send(ChatEvent::PeerJoined(name)).await?;
                    }
                    Some(old_name) => {
                        if old_name != name {
                            self.ui_event_tx
                                .send(ChatEvent::PeerNameChange {
                                    old: old_name,
                                    new: name,
                                })
                                .await?;
                        }
                    }
                }
            }
            MessageBody::Text { from, text } => {
                let name = self.state.resolve_name(from);
                self.ui_event_tx
                    .send(ChatEvent::MessageReceived {
                        author: name.to_string(),
                        content: text,
                    })
                    .await?;
            }
        }
        Ok(())
    }
}
