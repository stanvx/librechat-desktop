use std::convert::TryInto;

use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Nonce};
use anyhow::{bail, Context, Result};
use base64::{engine::general_purpose, Engine as _};
use keyring::Entry;
use rand::RngCore;

const NONCE_LENGTH: usize = 12;
const KEY_LENGTH: usize = 32;

/// Handles encryption key lifecycle backed by the operating system keychain.
#[derive(Debug, Clone)]
pub struct EncryptionManager {
    service: String,
    account: String,
}

impl EncryptionManager {
    pub fn new(service: impl Into<String>, account: impl Into<String>) -> Self {
        Self {
            service: service.into(),
            account: account.into(),
        }
    }

    fn entry(&self) -> Result<Entry> {
        Entry::new(&self.service, &self.account).context("failed to access system keychain entry")
    }

    /// Retrieves the encryption key from the keychain or generates a new one if missing.
    pub fn fetch_or_generate_key(&self) -> Result<[u8; KEY_LENGTH]> {
        let entry = self.entry()?;
        if let Ok(secret) = entry.get_password() {
            let decoded = general_purpose::STANDARD
                .decode(secret)
                .context("failed to decode stored encryption key")?;
            let key: [u8; KEY_LENGTH] = decoded
                .try_into()
                .map_err(|_| anyhow::anyhow!("stored encryption key has invalid length"))?;
            return Ok(key);
        }

        let mut key = [0u8; KEY_LENGTH];
        OsRng.fill_bytes(&mut key);
        let encoded = general_purpose::STANDARD.encode(key);
        entry
            .set_password(&encoded)
            .context("failed to persist encryption key to keychain")?;
        Ok(key)
    }

    /// Encrypts plaintext using AES-GCM with a randomly generated nonce.
    pub fn encrypt(&self, key: &[u8; KEY_LENGTH], plaintext: &[u8]) -> Result<Vec<u8>> {
        let cipher = Aes256Gcm::new_from_slice(key).context("failed to initialize cipher")?;
        let mut nonce_bytes = [0u8; NONCE_LENGTH];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        let mut ciphertext = cipher
            .encrypt(nonce, plaintext)
            .context("encryption failure")?;

        let mut output = Vec::with_capacity(NONCE_LENGTH + ciphertext.len());
        output.extend_from_slice(&nonce_bytes);
        output.append(&mut ciphertext);
        Ok(output)
    }

    /// Decrypts data previously produced by [`encrypt`].
    pub fn decrypt(&self, key: &[u8; KEY_LENGTH], data: &[u8]) -> Result<Vec<u8>> {
        if data.len() < NONCE_LENGTH {
            bail!("ciphertext too short to contain nonce");
        }
        let (nonce_bytes, ciphertext) = data.split_at(NONCE_LENGTH);
        let cipher = Aes256Gcm::new_from_slice(key).context("failed to initialize cipher")?;
        cipher
            .decrypt(Nonce::from_slice(nonce_bytes), ciphertext)
            .context("decryption failure")
    }
}
