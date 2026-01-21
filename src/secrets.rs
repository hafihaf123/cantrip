use anyhow::anyhow;
use anyhow::{Context, Result};
use argon2::Argon2;
use iroh::SecretKey;
use keyring::Entry;

pub fn get_secret_key(username: &str) -> Result<SecretKey> {
    let entry = Entry::new("cantrip-rs", username)
        .context("Unexpected error constructing a keychain entry")?;
    let secret = entry.get_secret();
    match secret {
        Ok(bytes) => (&bytes[..])
            .try_into()
            .context("Failed to construct a secret key from a value stored in the keychain."),
        Err(keyring::Error::NoEntry) => {
            let secret = SecretKey::generate(&mut rand::rng());
            entry
                .set_secret(&secret.to_bytes())
                .context("Failed to add the generated secret key to a keychain")?;
            Ok(secret)
        }
        Err(e) => Err(anyhow!(e)),
    }
}

pub fn hash_password(password: &str, topic_hash: &[u8]) -> [u8; 32] {
    let mut output_key = [0u8; 32];

    let params = argon2::Params::default();

    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
    argon2
        .hash_password_into(password.as_bytes(), topic_hash, &mut output_key)
        .expect("Failed to hash the password");

    output_key
}

