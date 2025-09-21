use serde::{Deserialize, Serialize};

use crate::models::{CacheType, Timestamp};

/// Represents a single encrypted cache record persisted locally.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncryptedCacheEntry {
    pub key: String,
    pub encrypted_data: Vec<u8>,
    pub created_at: Timestamp,
    pub accessed_at: Timestamp,
    pub expires_at: Option<Timestamp>,
    pub cache_type: CacheType,
    pub size_bytes: u64,
    pub encryption_key_id: String,
}

impl EncryptedCacheEntry {
    pub fn is_expired(&self, now: Timestamp) -> bool {
        self.expires_at.map_or(false, |expiry| expiry <= now)
    }
}
