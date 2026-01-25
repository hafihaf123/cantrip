use crate::events::{ChatEvent, NetworkEvent, SystemEvent};
use crate::message::{Message, MessageBody};
use anyhow::Result;
use futures_lite::StreamExt;
use iroh::{Endpoint, EndpointId, protocol::Router};
use iroh_gossip::api::{Event, GossipReceiver};
use std::collections::{HashMap, HashSet};
use tokio::sync::{broadcast, mpsc::Sender};

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

    fn remove_user(&mut self, author: &EndpointId) -> Option<String> {
        self.users.remove(author)
    }
}

pub struct ChatBackend {
    _endpoint: Endpoint,
    key: [u8; 32],
    state: ChatState,
    router: Router,
    receiver: GossipReceiver,
    event_tx: Sender<SystemEvent>,
    username: String,
    bad_actors: HashSet<EndpointId>,
}

impl ChatBackend {
    pub fn new(
        _endpoint: Endpoint,
        key: [u8; 32],
        router: Router,
        receiver: GossipReceiver,
        event_tx: Sender<SystemEvent>,
        username: String,
    ) -> Self {
        Self {
            _endpoint,
            key,
            state: ChatState::default(),
            router,
            receiver,
            event_tx,
            username,
            bad_actors: HashSet::new(),
        }
    }

    pub async fn subscribe_loop(mut self, mut shutdown_rx: broadcast::Receiver<()>) {
        loop {
            tokio::select! {
                _ = shutdown_rx.recv() => {
                    break;
                }

                event_option = self.receiver.try_next() => {
                    match event_option {
                        Ok(Some(event)) => {
                            if let Err(e) = self.handle_event(event).await
                                && self
                                    .event_tx
                                    .send(SystemEvent::Ui(ChatEvent::Error(format!(
                                        "Failed to process event: {:?}",
                                        e
                                    ))))
                                    .await
                                    .is_err()
                            {
                                break;
                            };
                        }
                        _ => break,
                    }
                }
            }
        }

        self.event_tx
            .send(SystemEvent::Ui(ChatEvent::SystemStatus(
                "You left the chat".to_string(),
            )))
            .await
            .ok();

        if let Err(e) = self.router.shutdown().await {
            self.event_tx
                .send(SystemEvent::Ui(ChatEvent::Error(format!(
                    "Error during shutdown: {:?}",
                    e
                ))))
                .await
                .expect("Failed to log shotdown error");
            // .ok();
        }
    }

    async fn handle_event(&mut self, event: Event) -> Result<()> {
        if let Event::Received(msg) = event {
            match Message::from_bytes(&msg.content)?.decrypt(&self.key) {
                Ok(decrypted) => {
                    if self.bad_actors.remove(&msg.delivered_from) {
                        self.event_tx
                            .send(SystemEvent::Ui(ChatEvent::SystemStatus(format!(
                                "Encryption recovered for peer {}",
                                msg.delivered_from.fmt_short()
                            ))))
                            .await?;
                    }
                    self.handle_message(decrypted).await?;
                }
                Err(e) => {
                    if self.bad_actors.insert(msg.delivered_from) {
                        self.event_tx
                            .send(SystemEvent::Ui(ChatEvent::Error(format!(
                                "Decryption failed for peer {}: {}. Usually caused by password or room name mismatch.",
                                msg.delivered_from.fmt_short(),
                                e
                            ))))
                            .await?;
                    }
                }
            };
        }
        Ok(())
    }

    async fn handle_message(&mut self, body: MessageBody) -> Result<()> {
        match body {
            MessageBody::Joined { from, name } => {
                match self.state.update_user(from, name.clone()) {
                    None => {
                        self.event_tx
                            .send(SystemEvent::Ui(ChatEvent::PeerJoined(name)))
                            .await?;
                        self.event_tx
                            .send(SystemEvent::Network(NetworkEvent::BroadcastJoin(
                                self.username.clone(),
                            )))
                            .await?;
                    }
                    Some(old_name) => {
                        if old_name != name {
                            self.event_tx
                                .send(SystemEvent::Ui(ChatEvent::PeerNameChange {
                                    old: old_name,
                                    new: name,
                                }))
                                .await?;
                        }
                    }
                }
            }
            MessageBody::Text { from, text } => {
                let name = self.state.resolve_name(from);
                self.event_tx
                    .send(SystemEvent::Ui(ChatEvent::MessageReceived {
                        author: name.to_string(),
                        content: text,
                    }))
                    .await?;
            }
            MessageBody::Left { from } => {
                if let Some(user) = self.state.remove_user(&from) {
                    self.event_tx
                        .send(SystemEvent::Ui(ChatEvent::PeerLeft(user)))
                        .await?;
                }
            }
            MessageBody::DiceRoll {
                from,
                result,
                dice,
                rolls,
            } => {
                let name = self.state.resolve_name(from);
                self.event_tx
                    .send(SystemEvent::Ui(ChatEvent::DiceRolled {
                        result,
                        rolls,
                        dice,
                        author: Some(name.to_string()),
                    }))
                    .await?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_name_resolution() {
        let mut state = ChatState::default();
        let user_id = EndpointId::from_bytes(&[0u8; 32]).unwrap();

        assert_eq!(state.resolve_name(user_id), "Unknown");

        state.update_user(user_id, "Alice".to_string());
        assert_eq!(state.resolve_name(user_id), "Alice");

        let old_name = state.update_user(user_id, "Bob".to_string());
        assert_eq!(old_name, Some("Alice".to_string()));
        assert_eq!(state.resolve_name(user_id), "Bob");
    }
}
