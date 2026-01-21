use anyhow::Result;
use iroh::{
    Endpoint, RelayMode,
    discovery::{dns::DnsDiscovery, mdns::MdnsDiscovery, pkarr::PkarrPublisher},
    protocol::Router,
};
use iroh_gossip::Gossip;
use tokio::sync::mpsc::{self, Sender};

use crate::chat::{ChatBackend, ChatClient, ChatConfig};
use crate::{events::ChatEvent, ticket::Ticket};

pub enum ChatCommand {
    BroadcastJoin,
}

pub struct ChatRoom {}

impl ChatRoom {
    pub async fn connect(
        config: ChatConfig,
        event_tx: Sender<ChatEvent>,
    ) -> Result<(ChatClient, ChatBackend)> {
        let endpoint = Endpoint::empty_builder(RelayMode::Default)
            .discovery(PkarrPublisher::n0_dns())
            .discovery(DnsDiscovery::n0_dns())
            .discovery(MdnsDiscovery::builder())
            .secret_key(config.secret_key)
            .bind()
            .await?;
        let gossip = Gossip::builder().spawn(endpoint.clone());
        let router = Router::builder(endpoint.clone())
            .accept(iroh_gossip::ALPN, gossip.clone())
            .spawn();

        let ticket = Ticket::new(config.topic, vec![endpoint.addr()]);
        event_tx
            .send(ChatEvent::SystemStatus(format!("ticket to join: {ticket}")))
            .await?;

        let endpoints = config.bootstrap_nodes;
        let endpoint_ids = endpoints.iter().map(|p| p.id).collect();
        if endpoints.is_empty() {
            event_tx
                .send(ChatEvent::SystemStatus(
                    "waiting for endpoints to join us...".to_string(),
                ))
                .await?;
        } else {
            event_tx
                .send(ChatEvent::SystemStatus(format!(
                    "trying to connect to {} endpoints...",
                    endpoints.len()
                )))
                .await?;
        };
        let (sender, receiver) = gossip
            .subscribe_and_join(config.topic, endpoint_ids)
            .await?
            .split();
        event_tx
            .send(ChatEvent::SystemStatus("connected!".to_string()))
            .await?;

        let client = ChatClient::new(sender, endpoint.clone(), config.symmetric_key);
        client.broadcast_join(config.username.clone()).await?;

        let (chat_tx, mut chat_rx) = mpsc::channel(100);

        let client_clone = client.clone();
        let event_tx_clone = event_tx.clone();
        tokio::spawn(async move {
            while let Some(command) = chat_rx.recv().await {
                match command {
                    ChatCommand::BroadcastJoin => {
                        if let Err(e) = client_clone.broadcast_join(config.username.clone()).await
                            && event_tx_clone
                                .send(ChatEvent::Error(format!(
                                    "Failed to broadcast a welcome message: {}",
                                    e
                                )))
                                .await
                                .is_err()
                        {
                            break;
                        }
                    }
                }
            }
        });

        let backend = ChatBackend::new(
            endpoint,
            config.symmetric_key,
            router,
            receiver,
            chat_tx,
            event_tx,
        );

        Ok((client, backend))
    }
}
