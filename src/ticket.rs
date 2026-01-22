use std::{fmt, str::FromStr};

use anyhow::{Result, anyhow};
use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use iroh::EndpointAddr;
use iroh_gossip::TopicId;
use serde::{Deserialize, Serialize};

pub const TICKET_PREFIX: &str = "ticket-";

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
        postcard::from_bytes(bytes).map_err(Into::into)
    }

    fn to_vec(&self) -> Vec<u8> {
        postcard::to_stdvec(self).expect("Unexpected error serializing a ticket")
    }
}

impl fmt::Display for Ticket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = BASE64_URL_SAFE_NO_PAD.encode(&self.to_vec()[..]);
        write!(f, "{}{}", TICKET_PREFIX, text)
    }
}

impl FromStr for Ticket {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let clean_string = s
            .strip_prefix(TICKET_PREFIX)
            .ok_or_else(|| anyhow!("Invalid ticket: missing '{}' prefix", TICKET_PREFIX))?;
        let bytes = BASE64_URL_SAFE_NO_PAD.decode(clean_string.as_bytes())?;
        Self::from_bytes(&bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iroh::EndpointAddr;
    use rand::rng;

    #[test]
    fn test_ticket_string_roundtrip() {
        let topic = TopicId::from_bytes([1u8; 32]);
        let node_id = iroh::SecretKey::generate(&mut rng()).public();
        let endpoint = EndpointAddr::from_parts(node_id, None);

        let ticket = Ticket::new(topic, vec![endpoint.clone()]);

        let ticket_str = ticket.to_string();

        let parsed_ticket = Ticket::from_str(&ticket_str).expect("Failed to parse ticket");

        assert_eq!(parsed_ticket.topic, topic);
        assert_eq!(parsed_ticket.endpoints.len(), 1);
        assert_eq!(parsed_ticket.endpoints.first().unwrap(), &endpoint);
    }
}
