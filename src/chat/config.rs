use crate::secrets::{get_secret_key, hash_password};
use crate::ticket::Ticket;
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
    pub is_host: bool,
}

impl ChatConfig {
    pub fn from_cli(cli: Cli) -> Result<Self> {
        let username = cli.username;

        let secret_key = get_secret_key(&username)?;

        let topic_hash = blake3::hash(cli.room.as_bytes());

        let (topic, bootstrap_nodes, is_host) = match &cli.ticket {
            None => {
                let topic = TopicId::from_bytes(rand::random());
                (topic, vec![], true)
            }
            Some(ticket) => {
                let (topic, endpoints) = Ticket::from_str(ticket)?.into_tuple();
                (topic, endpoints, false)
            }
        };

        let symmetric_key = hash_password(&cli.password, topic_hash.as_bytes());

        Ok(Self {
            username,
            secret_key,
            topic,
            bootstrap_nodes,
            symmetric_key,
            is_host,
        })
    }
}
