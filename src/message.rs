use anyhow::{Result, anyhow};
use chacha20poly1305::{
    AeadCore, KeyInit, XChaCha20Poly1305, XNonce,
    aead::{Aead, OsRng},
};
use iroh::EndpointId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    ciphertext: Vec<u8>,
    nonce: [u8; 24],
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageBody {
    Joined { from: EndpointId, name: String },
    Text { from: EndpointId, text: String },
}

impl Message {
    pub fn new(body: MessageBody, key: &[u8; 32]) -> Self {
        let nonce: [u8; 24] = XChaCha20Poly1305::generate_nonce(&mut OsRng).into();
        let ciphertext = body.encrypt(&nonce, key);
        Self { ciphertext, nonce }
    }

    pub fn decrypt(&self, key: &[u8; 32]) -> Result<MessageBody> {
        let cipher = XChaCha20Poly1305::new(key.into());
        let nonce = XNonce::from_slice(&self.nonce);

        let decrypt = cipher
            .decrypt(nonce, &self.ciphertext[..])
            .map_err(|_| anyhow!("Failed to decrypt a message"))?;

        MessageBody::from_bytes(&decrypt)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        serde_json::from_slice(bytes).map_err(Into::into)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Unexpected error serializing a message.")
    }
}

impl MessageBody {
    fn to_vec(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Unexpected error serializing a message body.")
    }

    fn encrypt(&self, nonce: &[u8; 24], key: &[u8; 32]) -> Vec<u8> {
        let cipher = XChaCha20Poly1305::new(key.into());

        cipher
            .encrypt(nonce.into(), &self.to_vec()[..])
            .expect("Unexpected failure while encrypting a message.")
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        serde_json::from_slice(bytes).map_err(Into::into)
    }
}
