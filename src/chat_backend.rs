use crate::{
    chatroom::ChatCommand,
    message::{Message, MessageBody},
};
use anyhow::Result;
use futures_lite::StreamExt;
use iroh::{Endpoint, EndpointId, protocol::Router};
use iroh_gossip::api::{Event, GossipReceiver};
use std::collections::HashMap;
use tokio::sync::mpsc;

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
    outbox: mpsc::Sender<ChatCommand>,
}

impl ChatBackend {
    pub fn new(
        _endpoint: Endpoint,
        key: [u8; 32],
        router: Router,
        receiver: GossipReceiver,
        outbox: mpsc::Sender<ChatCommand>,
    ) -> Self {
        Self {
            _endpoint,
            key,
            state: ChatState::default(),
            router,
            receiver,
            outbox,
        }
    }

    pub async fn subscribe_loop(mut self) {
        while let Some(event) = self.receiver.try_next().await.unwrap_or(None) {
            if let Err(e) = self.handle_event(event).await {
                eprintln!("Error processing event: {:?}", e);
            }
        }

        if let Err(e) = self.router.shutdown().await {
            eprintln!("Error during shutdown: {e:?}");
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
                        println!("> {} joined the room. Say hi!", name);
                        self.outbox.send(ChatCommand::BroadcastJoin).await?;
                    }
                    Some(old_name) => {
                        if old_name != name {
                            println!("> {} changed their name to {}", old_name, name);
                        }
                    }
                }
            }
            MessageBody::Text { from, text } => {
                let name = self.state.resolve_name(from);
                println!("{}: {}", name, text);
            }
        }
        Ok(())
    }
}
