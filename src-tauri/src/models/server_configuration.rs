use serde::{Deserialize, Serialize};

use crate::models::{AuthType, ConnectionStatus, Timestamp};

/// Stores LibreChat server configuration alongside authentication state.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerConfiguration {
    pub id: String,
    pub name: String,
    pub base_url: String,
    pub auth_type: AuthType,
    pub auth_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_expires_at: Option<Timestamp>,
    pub is_active: bool,
    pub is_secure: bool,
    pub last_connected: Option<Timestamp>,
    pub connection_status: ConnectionStatus,
    pub api_version: String,
    pub created_at: Timestamp,
}

impl ServerConfiguration {
    pub fn token_is_valid(&self, now: Timestamp) -> bool {
        self.token_expires_at.map_or(false, |expiry| expiry > now)
    }
}
