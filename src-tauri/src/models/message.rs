use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::{MessageRole, ProcessingState, SyncState, Timestamp};

/// Attachment metadata associated with a single message payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageAttachment {
    pub file_id: String,
    pub filename: String,
    pub mime_type: Option<String>,
    pub size_bytes: Option<u64>,
}

impl MessageAttachment {
    pub fn new(file_id: impl Into<String>, filename: impl Into<String>) -> Self {
        Self {
            file_id: file_id.into(),
            filename: filename.into(),
            mime_type: None,
            size_bytes: None,
        }
    }
}

/// Individual message exchanged within a conversation context.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: String,
    pub conversation_id: String,
    pub content: String,
    pub role: MessageRole,
    pub timestamp: Timestamp,
    pub sync_state: SyncState,
    #[serde(default)]
    pub attachments: Vec<MessageAttachment>,
    #[serde(default)]
    pub metadata: HashMap<String, Value>,
    pub processing_state: ProcessingState,
}

impl Message {
    pub fn add_attachment(&mut self, attachment: MessageAttachment) {
        self.attachments.push(attachment);
    }
}
