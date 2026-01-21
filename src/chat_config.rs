use crate::{
    cli::Command,
    secrets::{get_secret_key, hash_password},
    ticket::Ticket,
};
use anyhow::Result;
use iroh::{EndpointAddr, SecretKey};
use iroh_gossip::TopicId;
use std::str::FromStr;

use crate::cli::Cli;

pub struct ChatConfig {
    pub username: String,
    pub secret_key: SecretKey,
    pub topic: TopicId,
    pub bootstrap_nodes: Vec<EndpointAddr>,
    pub symmetric_key: [u8; 32],
}

impl ChatConfig {
    pub fn from_cli(cli: Cli) -> Result<Self> {
        let username = cli.username;

        let secret_key = get_secret_key(&username)?;

        let topic_hash = blake3::hash(cli.topic.as_bytes());

        let (topic, bootstrap_nodes) = match &cli.command {
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

        let symmetric_key = hash_password(&cli.password, topic_hash.as_bytes());

        Ok(Self {
            username,
            secret_key,
            topic,
            bootstrap_nodes,
            symmetric_key,
        })
    }
}
