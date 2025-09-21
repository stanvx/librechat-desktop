use std::str::FromStr;

use serde_json::from_str;
use sqlx::Row;
use thiserror::Error;

use crate::database::Database;
use crate::models::{CachePolicy, NotificationSettings, Theme, Timestamp, UserPreferences, WindowSettings};

#[derive(Debug, Error)]
pub enum PreferencesError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type PreferencesResult<T> = Result<T, PreferencesError>;

#[derive(Clone)]
pub struct PreferencesService {
    db: Database,
}

impl PreferencesService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn load(&self, user_id: &str) -> PreferencesResult<Option<UserPreferences>> {
        let row = sqlx::query(
            r#"
            SELECT user_id, global_hotkey, cache_policy, theme, window_settings,
                   notification_settings, quick_capture_enabled, system_tray_enabled,
                   auto_start, analytics_enabled, created_at, updated_at
            FROM user_preferences WHERE user_id = ?1
            "#,
        )
        .bind(user_id)
        .fetch_optional(self.db.pool())
        .await?;

        let Some(row) = row else { return Ok(None); };

        let window_settings: WindowSettings = from_str(row.get("window_settings"))?;
        let notification_settings: NotificationSettings = from_str(row.get("notification_settings"))?;

        let preferences = UserPreferences {
            user_id: row.get("user_id"),
            global_hotkey: row.get("global_hotkey"),
            cache_policy: CachePolicy::from_str(row.get::<String, _>("cache_policy").as_str())
                .unwrap_or(CachePolicy::Balanced),
            theme: Theme::from_str(row.get::<String, _>("theme").as_str()).unwrap_or_default(),
            window_settings,
            notification_settings,
            quick_capture_enabled: row.get("quick_capture_enabled"),
            system_tray_enabled: row.get("system_tray_enabled"),
            auto_start: row.get("auto_start"),
            analytics_enabled: row.get("analytics_enabled"),
            created_at: decode(row.get("created_at")),
            updated_at: decode(row.get("updated_at")),
        };

        Ok(Some(preferences))
    }

    pub async fn save(&self, preferences: &UserPreferences) -> PreferencesResult<()> {
        let window_settings = serde_json::to_string(&preferences.window_settings)?;
        let notification_settings = serde_json::to_string(&preferences.notification_settings)?;

        sqlx::query(
            r#"
            INSERT INTO user_preferences (
                user_id, global_hotkey, cache_policy, theme, window_settings,
                notification_settings, quick_capture_enabled, system_tray_enabled,
                auto_start, analytics_enabled, created_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
            ON CONFLICT(user_id) DO UPDATE SET
                global_hotkey = excluded.global_hotkey,
                cache_policy = excluded.cache_policy,
                theme = excluded.theme,
                window_settings = excluded.window_settings,
                notification_settings = excluded.notification_settings,
                quick_capture_enabled = excluded.quick_capture_enabled,
                system_tray_enabled = excluded.system_tray_enabled,
                auto_start = excluded.auto_start,
                analytics_enabled = excluded.analytics_enabled,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(&preferences.user_id)
        .bind(&preferences.global_hotkey)
        .bind(preferences.cache_policy.as_str())
        .bind(preferences.theme.as_str())
        .bind(window_settings)
        .bind(notification_settings)
        .bind(preferences.quick_capture_enabled)
        .bind(preferences.system_tray_enabled)
        .bind(preferences.auto_start)
        .bind(preferences.analytics_enabled)
        .bind(preferences.created_at.timestamp())
        .bind(preferences.updated_at.timestamp())
        .execute(self.db.pool())
        .await?;

        Ok(())
    }

    pub async fn update_hotkey(&self, user_id: &str, hotkey: Option<String>) -> PreferencesResult<()> {
        sqlx::query("UPDATE user_preferences SET global_hotkey = ?1, updated_at = ?2 WHERE user_id = ?3")
            .bind(hotkey)
            .bind(chrono::Utc::now().timestamp())
            .bind(user_id)
            .execute(self.db.pool())
            .await?;
        Ok(())
    }
}

fn decode(seconds: i64) -> Timestamp {
    chrono::Utc.timestamp_opt(seconds, 0).single().unwrap_or_else(|| chrono::Utc::now())
}
