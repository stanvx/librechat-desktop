use std::str::FromStr;

use once_cell::sync::OnceCell;
use sqlx::Row;
use thiserror::Error;

use crate::database::encryption::EncryptionManager;
use crate::database::Database;
use crate::models::{CacheType, EncryptedCacheEntry, Timestamp};

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("encryption error: {0}")]
    Encryption(String),
}

pub type CacheResult<T> = Result<T, CacheError>;

#[derive(Clone)]
pub struct CacheService {
    db: Database,
    encryption: EncryptionManager,
    key: std::sync::Arc<OnceCell<[u8; 32]>>,
}

impl CacheService {
    pub fn new(db: Database, encryption: EncryptionManager) -> Self {
        Self {
            db,
            encryption,
            key: std::sync::Arc::new(OnceCell::new()),
        }
    }

    async fn key(&self) -> CacheResult<[u8; 32]> {
        let manager = self.encryption.clone();
        let cell = self.key.clone();
        cell.get_or_try_init(|| manager.fetch_or_generate_key().map_err(|err| CacheError::Encryption(err.to_string())))
            .map(|value| *value)
    }

    pub async fn put(
        &self,
        cache_key: &str,
        cache_type: CacheType,
        plaintext: &[u8],
        expires_at: Option<Timestamp>,
    ) -> CacheResult<()> {
        let master_key = self.key().await?;
        let encrypted = self
            .encryption
            .encrypt(&master_key, plaintext)
            .map_err(|err| CacheError::Encryption(err.to_string()))?;

        let now = chrono::Utc::now();
        sqlx::query(
            r#"
            INSERT INTO encrypted_cache (
                cache_key, encrypted_data, created_at, accessed_at, expires_at,
                cache_type, size_bytes, encryption_key_id
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            ON CONFLICT(cache_key) DO UPDATE SET
                encrypted_data = excluded.encrypted_data,
                accessed_at = excluded.accessed_at,
                expires_at = excluded.expires_at,
                cache_type = excluded.cache_type,
                size_bytes = excluded.size_bytes
            "#,
        )
        .bind(cache_key)
        .bind(&encrypted)
        .bind(now.timestamp())
        .bind(now.timestamp())
        .bind(expires_at.map(|ts| ts.timestamp()))
        .bind(cache_type.as_str())
        .bind(plaintext.len() as i64)
        .bind("master-key")
        .execute(self.db.pool())
        .await?;

        Ok(())
    }

    pub async fn get(&self, cache_key: &str) -> CacheResult<Option<Vec<u8>>> {
        let row = sqlx::query(
            r#"
            SELECT encrypted_data FROM encrypted_cache WHERE cache_key = ?1
            "#,
        )
        .bind(cache_key)
        .fetch_optional(self.db.pool())
        .await?;

        let Some(row) = row else { return Ok(None); };
        let encrypted: Vec<u8> = row.get("encrypted_data");
        let master_key = self.key().await?;
        let plaintext = self
            .encryption
            .decrypt(&master_key, &encrypted)
            .map_err(|err| CacheError::Encryption(err.to_string()))?;

        sqlx::query("UPDATE encrypted_cache SET accessed_at = ?1 WHERE cache_key = ?2")
            .bind(chrono::Utc::now().timestamp())
            .bind(cache_key)
            .execute(self.db.pool())
            .await?;

        Ok(Some(plaintext))
    }

    pub async fn evict_expired(&self) -> CacheResult<u64> {
        let now = chrono::Utc::now().timestamp();
        let result = sqlx::query(
            "DELETE FROM encrypted_cache WHERE expires_at IS NOT NULL AND expires_at <= ?1",
        )
        .bind(now)
        .execute(self.db.pool())
        .await?;
        Ok(result.rows_affected())
    }

    pub async fn list_entries(&self) -> CacheResult<Vec<EncryptedCacheEntry>> {
        let rows = sqlx::query(
            r#"
            SELECT cache_key, encrypted_data, created_at, accessed_at, expires_at,
                   cache_type, size_bytes, encryption_key_id
            FROM encrypted_cache
            "#,
        )
        .fetch_all(self.db.pool())
        .await?;

        let mut entries = Vec::with_capacity(rows.len());
        for row in rows {
            let expires_at = row
                .get::<Option<i64>, _>("expires_at")
                .map(|ts| decode_timestamp(ts))
                .transpose()?;

            let entry = EncryptedCacheEntry {
                key: row.get("cache_key"),
                encrypted_data: row.get("encrypted_data"),
                created_at: decode_timestamp(row.get("created_at"))?,
                accessed_at: decode_timestamp(row.get("accessed_at"))?,
                expires_at,
                cache_type: CacheType::from_str(row.get::<String, _>("cache_type").as_str())
                    .map_err(CacheError::Encryption)?,
                size_bytes: row.get::<i64, _>("size_bytes") as u64,
                encryption_key_id: row.get("encryption_key_id"),
            };
            entries.push(entry);
        }

        Ok(entries)
    }
}

fn decode_timestamp(seconds: i64) -> CacheResult<Timestamp> {
    chrono::Utc
        .timestamp_opt(seconds, 0)
        .single()
        .ok_or_else(|| CacheError::Encryption("invalid timestamp".into()))
}
