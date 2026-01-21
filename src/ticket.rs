use std::{fmt, str::FromStr};

use anyhow::Result;
use iroh::EndpointAddr;
use iroh_gossip::TopicId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ticket {
    topic: TopicId,
    endpoints: Vec<EndpointAddr>,
}

impl Ticket {
    pub fn new(topic: TopicId, endpoints: Vec<EndpointAddr>) -> Self {
        Ticket { topic, endpoints }
    }

    pub fn into_tuple(self) -> (TopicId, Vec<EndpointAddr>) {
        (self.topic, self.endpoints)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        serde_json::from_slice(bytes).map_err(Into::into)
    }

    fn to_vec(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Unexpected error serializing a ticket")
    }
}

impl fmt::Display for Ticket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = data_encoding::BASE32_NOPAD.encode(&self.to_vec()[..]);
        text.make_ascii_lowercase();
        write!(f, "{}", text)
    }
}

impl FromStr for Ticket {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let bytes = data_encoding::BASE32_NOPAD.decode(s.to_ascii_uppercase().as_bytes())?;
        Self::from_bytes(&bytes)
    }
}
