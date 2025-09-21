use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Shared synchronization status for persisted records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncState {
    Local,
    Synced,
    Modified,
    Conflict,
    Error,
}

impl Default for SyncState {
    fn default() -> Self {
        Self::Local
    }
}

/// Cache size and retention policy tiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CachePolicy {
    Disabled,
    Lightweight,
    Balanced,
    Extended,
}

impl Default for CachePolicy {
    fn default() -> Self {
        Self::Balanced
    }
}

/// Denotes message author role when communicating with LibreChat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

impl Default for MessageRole {
    fn default() -> Self {
        Self::User
    }
}

/// Tracks backend processing lifecycle for messages and queue entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProcessingState {
    Pending,
    Processing,
    Complete,
    Failed,
    Cancelled,
}

impl Default for ProcessingState {
    fn default() -> Self {
        Self::Pending
    }
}

/// Authentication mechanism used for a LibreChat server.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthType {
    Jwt,
    Oauth,
    Ldap,
}

impl Default for AuthType {
    fn default() -> Self {
        Self::Jwt
    }
}

/// Connection state tracking for configured servers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
    Connecting,
    Error,
}

impl Default for ConnectionStatus {
    fn default() -> Self {
        Self::Disconnected
    }
}

/// Upload lifecycle for dropped files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UploadStatus {
    Pending,
    Uploading,
    Completed,
    Failed,
    Cancelled,
}

impl Default for UploadStatus {
    fn default() -> Self {
        Self::Pending
    }
}

/// Segment of encrypted cache to support targeted retention.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CacheType {
    Conversation,
    Message,
    File,
    Preference,
}

impl Default for CacheType {
    fn default() -> Self {
        Self::Conversation
    }
}

/// Visual theme preferences exposed to the UI layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    Light,
    Dark,
    System,
}

impl Default for Theme {
    fn default() -> Self {
        Self::System
    }
}

impl SyncState {
    pub const fn as_str(&self) -> &'static str {
        match self {
        Self::Local => "local",
        Self::Synced => "synced",
        Self::Modified => "modified",
        Self::Conflict => "conflict",
        Self::Error => "error",
        }
    }
}


impl FromStr for SyncState {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
        "local" => Ok(Self::Local),
        "synced" => Ok(Self::Synced),
        "modified" => Ok(Self::Modified),
        "conflict" => Ok(Self::Conflict),
        "error" => Ok(Self::Error),
            other => Err(format!("invalid SyncState: {}", other)),
        }
    }
}

impl CachePolicy {
    pub const fn as_str(&self) -> &'static str {
        match self {
        Self::Disabled => "disabled",
        Self::Lightweight => "lightweight",
        Self::Balanced => "balanced",
        Self::Extended => "extended",
        }
    }
}


impl FromStr for CachePolicy {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
        "disabled" => Ok(Self::Disabled),
        "lightweight" => Ok(Self::Lightweight),
        "balanced" => Ok(Self::Balanced),
        "extended" => Ok(Self::Extended),
            other => Err(format!("invalid CachePolicy: {}", other)),
        }
    }
}

impl MessageRole {
    pub const fn as_str(&self) -> &'static str {
        match self {
        Self::User => "user",
        Self::Assistant => "assistant",
        Self::System => "system",
        }
    }
}


impl FromStr for MessageRole {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
        "user" => Ok(Self::User),
        "assistant" => Ok(Self::Assistant),
        "system" => Ok(Self::System),
            other => Err(format!("invalid MessageRole: {}", other)),
        }
    }
}

impl ProcessingState {
    pub const fn as_str(&self) -> &'static str {
        match self {
        Self::Pending => "pending",
        Self::Processing => "processing",
        Self::Complete => "complete",
        Self::Failed => "failed",
        Self::Cancelled => "cancelled",
        }
    }
}


impl FromStr for ProcessingState {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
        "pending" => Ok(Self::Pending),
        "processing" => Ok(Self::Processing),
        "complete" => Ok(Self::Complete),
        "failed" => Ok(Self::Failed),
        "cancelled" => Ok(Self::Cancelled),
            other => Err(format!("invalid ProcessingState: {}", other)),
        }
    }
}

impl AuthType {
    pub const fn as_str(&self) -> &'static str {
        match self {
        Self::Jwt => "jwt",
        Self::Oauth => "oauth",
        Self::Ldap => "ldap",
        }
    }
}


impl FromStr for AuthType {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
        "jwt" => Ok(Self::Jwt),
        "oauth" => Ok(Self::Oauth),
        "ldap" => Ok(Self::Ldap),
            other => Err(format!("invalid AuthType: {}", other)),
        }
    }
}

impl ConnectionStatus {
    pub const fn as_str(&self) -> &'static str {
        match self {
        Self::Connected => "connected",
        Self::Disconnected => "disconnected",
        Self::Connecting => "connecting",
        Self::Error => "error",
        }
    }
}


impl FromStr for ConnectionStatus {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
        "connected" => Ok(Self::Connected),
        "disconnected" => Ok(Self::Disconnected),
        "connecting" => Ok(Self::Connecting),
        "error" => Ok(Self::Error),
            other => Err(format!("invalid ConnectionStatus: {}", other)),
        }
    }
}

impl UploadStatus {
    pub const fn as_str(&self) -> &'static str {
        match self {
        Self::Pending => "pending",
        Self::Uploading => "uploading",
        Self::Completed => "completed",
        Self::Failed => "failed",
        Self::Cancelled => "cancelled",
        }
    }
}


impl FromStr for UploadStatus {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
        "pending" => Ok(Self::Pending),
        "uploading" => Ok(Self::Uploading),
        "completed" => Ok(Self::Completed),
        "failed" => Ok(Self::Failed),
        "cancelled" => Ok(Self::Cancelled),
            other => Err(format!("invalid UploadStatus: {}", other)),
        }
    }
}

impl CacheType {
    pub const fn as_str(&self) -> &'static str {
        match self {
        Self::Conversation => "conversation",
        Self::Message => "message",
        Self::File => "file",
        Self::Preference => "preference",
        }
    }
}


impl FromStr for CacheType {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
        "conversation" => Ok(Self::Conversation),
        "message" => Ok(Self::Message),
        "file" => Ok(Self::File),
        "preference" => Ok(Self::Preference),
            other => Err(format!("invalid CacheType: {}", other)),
        }
    }
}

impl Theme {
    pub const fn as_str(&self) -> &'static str {
        match self {
        Self::Light => "light",
        Self::Dark => "dark",
        Self::System => "system",
        }
    }
}


impl FromStr for Theme {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
        "light" => Ok(Self::Light),
        "dark" => Ok(Self::Dark),
        "system" => Ok(Self::System),
            other => Err(format!("invalid Theme: {}", other)),
        }
    }
}
