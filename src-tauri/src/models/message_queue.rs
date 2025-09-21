use serde::{Deserialize, Serialize};

use crate::models::{ProcessingState, Timestamp};

/// File queued for upload alongside an offline message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueuedAttachment {
    pub file_path: String,
    pub mime_type: Option<String>,
    pub size_bytes: Option<u64>,
}

/// Offline queue entry that will be retried when connectivity is restored.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageQueueEntry {
    pub id: String,
    pub conversation_id: String,
    pub message_content: String,
    #[serde(default)]
    pub attachments: Vec<QueuedAttachment>,
    pub created_at: Timestamp,
    pub retry_count: u32,
    pub max_retries: u32,
    pub next_retry_at: Option<Timestamp>,
    pub error_message: Option<String>,
    pub processing_state: ProcessingState,
}

impl MessageQueueEntry {
    pub fn can_retry(&self, now: Timestamp) -> bool {
        self.retry_count < self.max_retries
            && self
                .next_retry_at
                .map(|timestamp| timestamp <= now)
                .unwrap_or(true)
    }
}
