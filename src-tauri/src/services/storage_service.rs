use std::collections::HashMap;
use std::str::FromStr;

use chrono::{TimeZone, Utc};
use serde_json::{Map, Value};
use sqlx::{sqlite::SqliteRow, Row};
use thiserror::Error;

use crate::database::Database;
use crate::models::{
    Conversation, Message, MessageAttachment, MessageQueueEntry, QueuedAttachment, SyncState,
    Timestamp,
};
use crate::models::{CachePolicy, MessageRole, ProcessingState};

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("invalid enum value: {0}")]
    InvalidEnum(String),
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("invalid timestamp for {0}")]
    InvalidTimestamp(&'static str),
}

pub type StorageResult<T> = Result<T, StorageError>;

#[derive(Clone)]
pub struct StorageService {
    db: Database,
}

impl StorageService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn upsert_conversation(&self, conversation: &Conversation) -> StorageResult<()> {
        sqlx::query(
            r#"
            INSERT INTO conversations (
                id, title, created_at, updated_at, is_pinned, is_starred, server_id,
                sync_state, cache_policy, message_count, last_message_preview
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            ON CONFLICT(id) DO UPDATE SET
                title = excluded.title,
                updated_at = excluded.updated_at,
                is_pinned = excluded.is_pinned,
                is_starred = excluded.is_starred,
                server_id = excluded.server_id,
                sync_state = excluded.sync_state,
                cache_policy = excluded.cache_policy,
                message_count = excluded.message_count,
                last_message_preview = excluded.last_message_preview
            "#,
        )
        .bind(&conversation.id)
        .bind(&conversation.title)
        .bind(encode_timestamp(conversation.created_at))
        .bind(encode_timestamp(conversation.updated_at))
        .bind(conversation.is_pinned)
        .bind(conversation.is_starred)
        .bind(&conversation.server_id)
        .bind(conversation.sync_state.as_str())
        .bind(conversation.cache_policy.as_str())
        .bind(conversation.message_count as i64)
        .bind(&conversation.last_message_preview)
        .execute(self.db.pool())
        .await?;
        Ok(())
    }

    pub async fn get_conversation(&self, conversation_id: &str) -> StorageResult<Option<Conversation>> {
        let row = sqlx::query(
            r#"
            SELECT id, title, created_at, updated_at, is_pinned, is_starred, server_id,
                   sync_state, cache_policy, message_count, last_message_preview
            FROM conversations
            WHERE id = ?1
            "#,
        )
        .bind(conversation_id)
        .fetch_optional(self.db.pool())
        .await?;

        row.map(|row| map_row_to_conversation(&row)).transpose()
    }

    pub async fn list_conversations(
        &self,
        limit: u32,
        offset: u32,
    ) -> StorageResult<Vec<Conversation>> {
        let rows = sqlx::query(
            r#"
            SELECT id, title, created_at, updated_at, is_pinned, is_starred, server_id,
                   sync_state, cache_policy, message_count, last_message_preview
            FROM conversations
            ORDER BY updated_at DESC
            LIMIT ?1 OFFSET ?2
            "#,
        )
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(self.db.pool())
        .await?;

        rows.into_iter()
            .map(|row| map_row_to_conversation(&row))
            .collect()
    }

    pub async fn remove_conversation(&self, conversation_id: &str) -> StorageResult<()> {
        sqlx::query("DELETE FROM conversations WHERE id = ?1")
            .bind(conversation_id)
            .execute(self.db.pool())
            .await?;
        Ok(())
    }

    pub async fn upsert_message(&self, message: &Message) -> StorageResult<()> {
        let mut tx = self.db.pool().begin().await?;

        sqlx::query(
            r#"
            INSERT INTO messages (
                id, conversation_id, content, role, timestamp, sync_state,
                metadata, processing_state
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            ON CONFLICT(id) DO UPDATE SET
                content = excluded.content,
                role = excluded.role,
                timestamp = excluded.timestamp,
                sync_state = excluded.sync_state,
                metadata = excluded.metadata,
                processing_state = excluded.processing_state
            "#,
        )
        .bind(&message.id)
        .bind(&message.conversation_id)
        .bind(&message.content)
        .bind(message.role.as_str())
        .bind(encode_timestamp(message.timestamp))
        .bind(message.sync_state.as_str())
        .bind(serde_json::to_string(&message.metadata)?)
        .bind(message.processing_state.as_str())
        .execute(&mut *tx)
        .await?;

        sqlx::query("DELETE FROM message_attachments WHERE message_id = ?1")
            .bind(&message.id)
            .execute(&mut *tx)
            .await?;

        for attachment in &message.attachments {
            sqlx::query(
                r#"
                INSERT INTO message_attachments (
                    message_id, file_id, filename, mime_type, size_bytes
                ) VALUES (?1, ?2, ?3, ?4, ?5)
                "#,
            )
            .bind(&message.id)
            .bind(&attachment.file_id)
            .bind(&attachment.filename)
            .bind(&attachment.mime_type)
            .bind(attachment.size_bytes.map(|value| value as i64))
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn get_message(&self, message_id: &str) -> StorageResult<Option<Message>> {
        let row = sqlx::query(
            r#"
            SELECT id, conversation_id, content, role, timestamp, sync_state,
                   metadata, processing_state
            FROM messages WHERE id = ?1
            "#,
        )
        .bind(message_id)
        .fetch_optional(self.db.pool())
        .await?;

        let Some(row) = row else { return Ok(None); };
        let mut message = map_row_to_message(&row)?;

        let attachments = sqlx::query(
            r#"
            SELECT file_id, filename, mime_type, size_bytes
            FROM message_attachments WHERE message_id = ?1
            "#,
        )
        .bind(message_id)
        .fetch_all(self.db.pool())
        .await?;

        message.attachments = attachments
            .into_iter()
            .map(|row| MessageAttachment {
                file_id: row.get("file_id"),
                filename: row.get("filename"),
                mime_type: row.get("mime_type"),
                size_bytes: row.get::<Option<i64>, _>("size_bytes").map(|value| value as u64),
            })
            .collect();

        Ok(Some(message))
    }

    pub async fn enqueue_message(&self, entry: &MessageQueueEntry) -> StorageResult<()> {
        let mut tx = self.db.pool().begin().await?;

        sqlx::query(
            r#"
            INSERT INTO message_queue (
                id, conversation_id, message_content, created_at, retry_count,
                max_retries, next_retry_at, error_message, processing_state
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            ON CONFLICT(id) DO UPDATE SET
                message_content = excluded.message_content,
                retry_count = excluded.retry_count,
                max_retries = excluded.max_retries,
                next_retry_at = excluded.next_retry_at,
                error_message = excluded.error_message,
                processing_state = excluded.processing_state
            "#,
        )
        .bind(&entry.id)
        .bind(&entry.conversation_id)
        .bind(&entry.message_content)
        .bind(encode_timestamp(entry.created_at))
        .bind(entry.retry_count as i64)
        .bind(entry.max_retries as i64)
        .bind(entry.next_retry_at.map(encode_timestamp))
        .bind(&entry.error_message)
        .bind(entry.processing_state.as_str())
        .execute(&mut *tx)
        .await?;

        sqlx::query("DELETE FROM message_queue_attachments WHERE queue_id = ?1")
            .bind(&entry.id)
            .execute(&mut *tx)
            .await?;

        for attachment in &entry.attachments {
            sqlx::query(
                r#"
                INSERT INTO message_queue_attachments (
                    queue_id, file_path, mime_type, size_bytes
                ) VALUES (?1, ?2, ?3, ?4)
                "#,
            )
            .bind(&entry.id)
            .bind(&attachment.file_path)
            .bind(&attachment.mime_type)
            .bind(attachment.size_bytes.map(|value| value as i64))
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn list_pending_queue(&self, now: Timestamp) -> StorageResult<Vec<MessageQueueEntry>> {
        let rows = sqlx::query(
            r#"
            SELECT id, conversation_id, message_content, created_at, retry_count,
                   max_retries, next_retry_at, error_message, processing_state
            FROM message_queue
            WHERE retry_count < max_retries
              AND (next_retry_at IS NULL OR next_retry_at <= ?1)
            "#,
        )
        .bind(encode_timestamp(now))
        .fetch_all(self.db.pool())
        .await?;

        let mut entries = Vec::with_capacity(rows.len());
        for row in rows {
            let mut entry = map_row_to_queue_entry(&row)?;
            let attachments = sqlx::query(
                r#"
                SELECT file_path, mime_type, size_bytes
                FROM message_queue_attachments WHERE queue_id = ?1
                "#,
            )
            .bind(&entry.id)
            .fetch_all(self.db.pool())
            .await?;

            entry.attachments = attachments
                .into_iter()
                .map(|row| QueuedAttachment {
                    file_path: row.get("file_path"),
                    mime_type: row.get("mime_type"),
                    size_bytes: row.get::<Option<i64>, _>("size_bytes").map(|value| value as u64),
                })
                .collect();

            entries.push(entry);
        }



    pub async fn delete_queue_entry(&self, queue_id: &str) -> StorageResult<()> {
        sqlx::query("DELETE FROM message_queue WHERE id = ?1")
            .bind(queue_id)
            .execute(self.db.pool())
            .await?;
        Ok(())
    }

    pub async fn update_queue_retry(
        &self,
        queue_id: &str,
        retry_count: u32,
        next_retry_at: Option<Timestamp>,
        error_message: Option<String>,
        state: ProcessingState,
    ) -> StorageResult<()> {
        sqlx::query(
            r#"UPDATE message_queue
               SET retry_count = ?1,
                   next_retry_at = ?2,
                   error_message = ?3,
                   processing_state = ?4
               WHERE id = ?5"#,
        )
        .bind(retry_count as i64)
        .bind(next_retry_at.map(encode_timestamp))
        .bind(error_message)
        .bind(state.as_str())
        .bind(queue_id)
        .execute(self.db.pool())
        .await?;
        Ok(())
    }
        Ok(entries)
    }
}

fn encode_timestamp(ts: Timestamp) -> i64 {
    ts.timestamp()
}

fn decode_timestamp(seconds: i64, field: &'static str) -> StorageResult<Timestamp> {
    Utc.timestamp_opt(seconds, 0)
        .single()
        .ok_or(StorageError::InvalidTimestamp(field))
}

fn map_row_to_conversation(row: &SqliteRow) -> StorageResult<Conversation> {
    Ok(Conversation {
        id: row.get("id"),
        title: row.get("title"),
        created_at: decode_timestamp(row.get("created_at"), "created_at")?,
        updated_at: decode_timestamp(row.get("updated_at"), "updated_at")?,
        is_pinned: row.get("is_pinned"),
        is_starred: row.get("is_starred"),
        server_id: row.get("server_id"),
        sync_state: SyncState::from_str(row.get::<String, _>("sync_state").as_str())
            .map_err(StorageError::InvalidEnum)?,
        cache_policy: CachePolicy::from_str(row.get::<String, _>("cache_policy").as_str())
            .map_err(StorageError::InvalidEnum)?,
        message_count: row.get::<i64, _>("message_count") as u32,
        last_message_preview: row.get("last_message_preview"),
    })
}

fn map_row_to_message(row: &SqliteRow) -> StorageResult<Message> {
    let metadata_json: String = row.get("metadata");
    let map: Map<String, Value> = serde_json::from_str(&metadata_json)?;
    let metadata: HashMap<String, Value> = map.into_iter().collect();

    Ok(Message {
        id: row.get("id"),
        conversation_id: row.get("conversation_id"),
        content: row.get("content"),
        role: MessageRole::from_str(row.get::<String, _>("role").as_str())
            .map_err(StorageError::InvalidEnum)?,
        timestamp: decode_timestamp(row.get("timestamp"), "timestamp")?,
        sync_state: SyncState::from_str(row.get::<String, _>("sync_state").as_str())
            .map_err(StorageError::InvalidEnum)?,
        attachments: Vec::new(),
        metadata,
        processing_state: ProcessingState::from_str(
            row.get::<String, _>("processing_state").as_str(),
        )
        .map_err(StorageError::InvalidEnum)?,
    })
}

fn map_row_to_queue_entry(row: &SqliteRow) -> StorageResult<MessageQueueEntry> {
    Ok(MessageQueueEntry {
        id: row.get("id"),
        conversation_id: row.get("conversation_id"),
        message_content: row.get("message_content"),
        attachments: Vec::new(),
        created_at: decode_timestamp(row.get("created_at"), "created_at")?,
        retry_count: row.get::<i64, _>("retry_count") as u32,
        max_retries: row.get::<i64, _>("max_retries") as u32,
        next_retry_at: row
            .get::<Option<i64>, _>("next_retry_at")
            .map(|seconds| decode_timestamp(seconds, "next_retry_at"))
            .transpose()?,
        error_message: row.get("error_message"),
        processing_state: ProcessingState::from_str(
            row.get::<String, _>("processing_state").as_str(),
        )
        .map_err(StorageError::InvalidEnum)?,
    })
}
