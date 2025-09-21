use serde::{Deserialize, Serialize};

use crate::models::{CachePolicy, Theme, Timestamp};

/// Stores persisted window sizing and positioning preferences.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowSettings {
    pub width: u32,
    pub height: u32,
    pub position_x: Option<i32>,
    pub position_y: Option<i32>,
    pub is_maximized: bool,
    pub is_fullscreen: bool,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            width: 1024,
            height: 768,
            position_x: None,
            position_y: None,
            is_maximized: false,
            is_fullscreen: false,
        }
    }
}

/// Notification configuration covering desktop prompts and sounds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationSettings {
    pub enabled: bool,
    pub play_sound: bool,
    pub show_alerts: bool,
}

impl Default for NotificationSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            play_sound: true,
            show_alerts: true,
        }
    }
}

/// Captures per-user preferences stored locally for the desktop client.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPreferences {
    pub user_id: String,
    pub global_hotkey: Option<String>,
    pub cache_policy: CachePolicy,
    pub theme: Theme,
    #[serde(default)]
    pub window_settings: WindowSettings,
    #[serde(default)]
    pub notification_settings: NotificationSettings,
    pub quick_capture_enabled: bool,
    pub system_tray_enabled: bool,
    pub auto_start: bool,
    pub analytics_enabled: bool,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl UserPreferences {
    pub fn touch(&mut self, now: Timestamp) {
        self.updated_at = now;
    }
}
