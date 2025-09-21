use std::collections::HashMap;
use std::str::FromStr;

use chrono::Utc;
use thiserror::Error;

use super::messages_service::{Message as RemoteMessage, SendMessageRequest};
use super::{ApiError, ConversationsService, MessagesService};
use crate::models::{
    CachePolicy, Conversation, Message, MessageAttachment, MessageQueueEntry, MessageRole,
    ProcessingState, SyncState, Timestamp,
};
use crate::services::storage_service::{StorageError, StorageService};

#[derive(Debug, Error)]
pub enum SyncError {
    #[error("storage error: {0}")]
    Storage(#[from] StorageError),
    #[error("api error: {0}")]
    Api(#[from] ApiError),
    #[error("invalid data: {0}")]
    InvalidData(String),
}

pub type SyncResult<T> = Result<T, SyncError>;

#[derive(Clone)]
pub struct SyncService {
    storage: StorageService,
    conversations: ConversationsService,
    messages: MessagesService,
}

impl SyncService {
    pub fn new(
        storage: StorageService,
        conversations: ConversationsService,
        messages: MessagesService,
    ) -> Self {
        Self {
            storage,
            conversations,
            messages,
        }
    }

    /// Pulls the latest representation of a conversation and persists it locally.
    pub async fn sync_conversation(&self, server_id: &str, conversation_id: &str) -> SyncResult<()> {
        let remote = self.conversations.get(conversation_id).await?;
        let conversation = map_remote_conversation(&remote.summary, server_id)?;
        self.storage.upsert_conversation(&conversation).await?;

        for message in remote.messages {
            let local = map_remote_message(&message)?;
            self.storage.upsert_message(&local).await?;
        }

        Ok(())
    }

    /// Attempts to replay any queued offline messages.
    pub async fn process_outbox(&self) -> SyncResult<()> {
        let now = Utc::now();
        let queue = self.storage.list_pending_queue(now).await?;

        for entry in queue {
            match self.replay_queue_entry(&entry).await {
                Ok(_) => {
                    self.storage.delete_queue_entry(&entry.id).await?;
                }
                Err(err) => {
                    let next_retry = now + chrono::Duration::minutes(5);
                    let retries = entry.retry_count + 1;
                    self.storage
                        .update_queue_retry(
                            &entry.id,
                            retries,
                            Some(next_retry),
                            Some(format!("{}", err)),
                            ProcessingState::Failed,
                        )
                        .await?;
                }
            }
        }

        Ok(())
    }

    async fn replay_queue_entry(&self, entry: &MessageQueueEntry) -> SyncResult<()> {
        let request = SendMessageRequest {
            text: entry.message_content.clone(),
            conversation_id: entry.conversation_id.clone(),
            parent_message_id: None,
            model: None,
            endpoint: None,
            files: if entry.attachments.is_empty() {
                None
            } else {
                Some(entry.attachments.iter().map(|att| att.file_path.clone()).collect())
            },
            preset_id: None,
        };

        self.messages.send(&request).await?;
        Ok(())
    }
}

fn map_remote_conversation(
    summary: &super::conversations_service::ConversationSummary,
    server_id: &str,
) -> SyncResult<Conversation> {
    Ok(Conversation {
        id: summary.conversation_id.clone(),
        title: summary.title.clone(),
        created_at: parse_timestamp(&summary.created_at, "created_at")?,
        updated_at: parse_timestamp(&summary.updated_at, "updated_at")?,
        is_pinned: summary.is_pinned,
        is_starred: summary.is_starred,
        server_id: server_id.to_string(),
        sync_state: SyncState::Synced,
        cache_policy: CachePolicy::Balanced,
        message_count: 0,
        last_message_preview: None,
    })
}

fn map_remote_message(remote: &RemoteMessage) -> SyncResult<Message> {
    let timestamp = parse_timestamp(&remote.created_at, "created_at")?;
    let role = MessageRole::from_str(remote.sender.as_str())
        .map_err(|_| SyncError::InvalidData(format!("unexpected sender: {}", remote.sender)))?;

    let attachments = remote
        .files
        .iter()
        .map(|file| MessageAttachment {
            file_id: file.file_id.clone(),
            filename: file.filename.clone(),
            mime_type: file.r#type.clone(),
            size_bytes: file.size,
        })
        .collect();

    Ok(Message {
        id: remote.message_id.clone(),
        conversation_id: remote.conversation_id.clone(),
        content: remote.text.clone(),
        role,
        timestamp,
        sync_state: SyncState::Synced,
        attachments,
        metadata: HashMap::new(),
        processing_state: ProcessingState::Complete,
    })
}

fn parse_timestamp(value: &str, field: &'static str) -> SyncResult<Timestamp> {
    let parsed = chrono::DateTime::parse_from_rfc3339(value)
        .map_err(|_| SyncError::InvalidData(format!("invalid {} timestamp: {}", field, value)))?;
    Ok(parsed.with_timezone(&Utc))
}
