use anyhow::Result;
use iroh::{
    Endpoint, RelayMode,
    discovery::{dns::DnsDiscovery, mdns::MdnsDiscovery, pkarr::PkarrPublisher},
    protocol::Router,
};
use iroh_gossip::{Gossip, TopicId};
use std::str::FromStr;
use tokio::sync::mpsc;

use crate::{
    chat_backend::ChatBackend,
    chat_client::ChatClient,
    cli::{Cli, Command},
    secrets::{get_secret_key, hash_password},
    ticket::Ticket,
};

pub enum ChatCommand {
    BroadcastJoin,
}

pub struct ChatRoom {}

impl ChatRoom {
    pub async fn join(cli: Cli) -> Result<(ChatClient, ChatBackend)> {
        let username = cli.username;

        let secret_key = get_secret_key(&username)?;
        let endpoint = Endpoint::empty_builder(RelayMode::Default)
            .discovery(PkarrPublisher::n0_dns())
            .discovery(DnsDiscovery::n0_dns())
            .discovery(MdnsDiscovery::builder())
            .secret_key(secret_key)
            .bind()
            .await?;
        let gossip = Gossip::builder().spawn(endpoint.clone());
        let router = Router::builder(endpoint.clone())
            .accept(iroh_gossip::ALPN, gossip.clone())
            .spawn();

        let topic_hash = blake3::hash(cli.topic.as_bytes());

        let (topic, endpoints) = match &cli.command {
            Command::Open => {
                let topic = TopicId::from_bytes(rand::random());
                println!("> opening chat room for topic {topic}");
                (topic, vec![])
            }
            Command::Join { ticket } => {
                let (topic, endpoints) = Ticket::from_str(ticket)?.into_tuple();
                println!("> joining chat room for topic {topic}");
                (topic, endpoints)
            }
        };

        let ticket = Ticket::new(topic, vec![endpoint.addr()]);
        println!("> ticket to join: {ticket}");

        let endpoint_ids = endpoints.iter().map(|p| p.id).collect();
        if endpoints.is_empty() {
            println!("> waiting for endpoints to join us...");
        } else {
            println!("> trying to connect to {} endpoints...", endpoints.len());
        };
        let (sender, receiver) = gossip
            .subscribe_and_join(topic, endpoint_ids)
            .await?
            .split();
        println!("> connected!");

        let key = hash_password(&cli.password, topic_hash.as_bytes());

        let client = ChatClient::new(sender, endpoint.clone(), key);
        client.broadcast_join(username.clone()).await?;

        let (chat_tx, mut chat_rx) = mpsc::channel(100);

        let client_clone = client.clone();
        tokio::spawn(async move {
            while let Some(command) = chat_rx.recv().await {
                match command {
                    ChatCommand::BroadcastJoin => client_clone
                        .broadcast_join(username.clone())
                        .await
                        .unwrap_or_else(|_| eprintln!("Failed to broadcast a welcome message.")),
                }
            }
        });

        let backend = ChatBackend::new(endpoint, key, router, receiver, chat_tx);

        Ok((client, backend))
    }
}
