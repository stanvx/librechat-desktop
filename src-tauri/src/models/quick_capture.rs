use serde::{Deserialize, Serialize};

use crate::models::Timestamp;

/// Lightweight representation of a quick capture overlay session.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickCaptureSession {
    pub id: String,
    pub query: String,
    pub response: Option<String>,
    pub created_at: Timestamp,
    pub completed_at: Option<Timestamp>,
    pub session_duration_ms: Option<u64>,
    pub converted_to_conversation: Option<String>,
    pub server_id: String,
}

impl QuickCaptureSession {
    pub fn mark_completed(&mut self, timestamp: Timestamp, response: Option<String>) {
        self.response = response;
        self.completed_at = Some(timestamp);
        let duration_ms = timestamp
            .signed_duration_since(self.created_at)
            .num_milliseconds();
        self.session_duration_ms = Some(if duration_ms.is_negative() {
            0
        } else {
            duration_ms as u64
        });
    }
}
