use anyhow::{Context, Result, anyhow};
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
    /// Creates a new message from the message body by encrypting it with the key and a random
    /// generated nonce
    pub fn new(body: MessageBody, key: &[u8; 32]) -> Result<Self> {
        let nonce: [u8; 24] = XChaCha20Poly1305::generate_nonce(&mut OsRng).into();
        let ciphertext = body.encrypt(&nonce, key)?;
        Ok(Self { ciphertext, nonce })
    }

    /// Tries to decrypt a message body using the saved nonce and a specified key
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

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        serde_json::to_vec(self).context("Unexpected error serializing a message.")
    }
}

impl MessageBody {
    fn to_vec(&self) -> Result<Vec<u8>> {
        serde_json::to_vec(self).context("Unexpected error serializing a message body.")
    }

    /// Encrypts the message body using the specified key and nonce
    fn encrypt(&self, nonce: &[u8; 24], key: &[u8; 32]) -> Result<Vec<u8>> {
        let cipher = XChaCha20Poly1305::new(key.into());

        cipher
            .encrypt(nonce.into(), &self.to_vec()?[..])
            .map_err(|_| anyhow!("Unexpected failure while encrypting a message."))
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        serde_json::from_slice(bytes).map_err(Into::into)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use iroh::EndpointId;

    fn test_key() -> [u8; 32] {
        [42u8; 32]
    }

    fn test_id() -> EndpointId {
        EndpointId::from_bytes(&[0u8; 32]).unwrap()
    }

    fn test_text() -> String {
        String::from("TEST string -/&#\\/(546123;`é")
    }

    fn test_message() -> Message {
        let body = MessageBody::Text {
            from: test_id(),
            text: test_text(),
        };
        Message::new(body, &test_key()).unwrap()
    }

    #[test]
    fn test_encrypt_decrypt() {
        let message = test_message();
        let decrypted = message.decrypt(&test_key()).unwrap();

        if let MessageBody::Text { from, text } = decrypted {
            assert_eq!(from, test_id());
            assert_eq!(text, test_text());
        } else {
            panic!("Decryption went wrong")
        }
    }

    #[test]
    fn test_decrypt_fail_wrong_key() {
        let wrong_key = test_key().map(|u| u.wrapping_add(1));

        let message = test_message();
        let decrypted_res = message.decrypt(&wrong_key);

        assert!(decrypted_res.is_err());
    }
}
