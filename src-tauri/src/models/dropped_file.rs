use serde::{Deserialize, Serialize};

use crate::models::{Timestamp, UploadStatus};

/// Metadata for files users drag into the desktop client prior to upload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DroppedFile {
    pub id: String,
    pub conversation_id: Option<String>,
    pub original_name: String,
    pub file_path: String,
    pub mime_type: String,
    pub size_bytes: u64,
    pub checksum: String,
    pub upload_status: UploadStatus,
    pub server_file_id: Option<String>,
    pub dropped_at: Timestamp,
    pub processed_at: Option<Timestamp>,
    pub error_message: Option<String>,
}

impl DroppedFile {
    pub fn mark_processed(&mut self, timestamp: Timestamp) {
        self.upload_status = UploadStatus::Completed;
        self.processed_at = Some(timestamp);
    }
}
