use serde::{Deserialize, Serialize};

use crate::models::{CachePolicy, SyncState, Timestamp};

/// Primary record representing a LibreChat conversation with metadata required for offline caching.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Conversation {
    pub id: String,
    pub title: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub is_pinned: bool,
    pub is_starred: bool,
    pub server_id: String,
    pub sync_state: SyncState,
    pub cache_policy: CachePolicy,
    pub message_count: u32,
    pub last_message_preview: Option<String>,
}

impl Conversation {
    /// Convenience helper for bumping message counters when new content arrives.
    pub fn increment_message_count(&mut self) {
        self.message_count = self.message_count.saturating_add(1);
    }
}
