# Data Model: LibreChat Desktop Application

## Core Entities

### 1. Conversation

**Purpose**: Represents a chat session with LibreChat

**Fields**:
- `id: String` - Unique conversation identifier (UUID or LibreChat ID)
- `title: String` - Conversation title/summary
- `created_at: DateTime` - Creation timestamp
- `updated_at: DateTime` - Last modification timestamp
- `is_pinned: bool` - Whether conversation is pinned for priority access
- `is_starred: bool` - Whether conversation is starred
- `server_id: String` - Reference to server configuration
- `sync_state: SyncState` - Offline synchronization status
- `cache_policy: CachePolicy` - Retention policy for this conversation
- `message_count: u32` - Number of messages in conversation
- `last_message_preview: Option<String>` - Preview of last message for UI

**Relationships**:
- Has many Messages (1:N)
- Belongs to ServerConfiguration (N:1)
- Has many DroppedFiles (1:N)

**Validation Rules**:
- `id` must be non-empty string
- `title` max length 200 characters
- `created_at` <= `updated_at`
- `message_count` >= 0

**State Transitions**:
```
Created → Active → [Pinned/Starred] → Archived → Deleted
     ↓      ↓           ↓              ↓         ↓
  Synced  Synced    Synced         Synced   Local Only
```

### 2. Message

**Purpose**: Individual message within a conversation

**Fields**:
- `id: String` - Unique message identifier
- `conversation_id: String` - Parent conversation reference
- `content: String` - Message text content
- `role: MessageRole` - Enum: User, Assistant, System
- `timestamp: DateTime` - Message creation time
- `sync_state: SyncState` - Synchronization status
- `attachments: Vec<MessageAttachment>` - File attachments
- `metadata: HashMap<String, Value>` - Extensible metadata (model info, tokens, etc.)
- `processing_state: ProcessingState` - Message processing status

**Relationships**:
- Belongs to Conversation (N:1)
- Has many MessageAttachments (1:N)

**Validation Rules**:
- `content` max length 32,768 characters
- `role` must be valid enum value
- `conversation_id` must reference existing conversation

### 3. EncryptedCache

**Purpose**: Encrypted local storage management

**Fields**:
- `key: String` - Storage key/identifier
- `encrypted_data: Vec<u8>` - AES-256 encrypted content
- `created_at: DateTime` - Cache entry creation time
- `accessed_at: DateTime` - Last access timestamp
- `expires_at: Option<DateTime>` - Optional expiration time
- `cache_type: CacheType` - Enum: Conversation, Message, File, Preference
- `size_bytes: u64` - Unencrypted content size
- `encryption_key_id: String` - Reference to encryption key in OS keychain

**Validation Rules**:
- `key` must be non-empty and unique
- `size_bytes` > 0
- `accessed_at` >= `created_at`
- `expires_at` if present must be > `created_at`

### 4. MessageQueue

**Purpose**: Offline message queue for connectivity loss scenarios

**Fields**:
- `id: String` - Queue entry identifier
- `conversation_id: String` - Target conversation
- `message_content: String` - Message to be sent
- `attachments: Vec<QueuedAttachment>` - Files to upload
- `created_at: DateTime` - Queue entry timestamp
- `retry_count: u32` - Number of send attempts
- `max_retries: u32` - Maximum retry attempts (default: 3)
- `next_retry_at: Option<DateTime>` - Next retry timestamp
- `error_message: Option<String>` - Last error if failed

**Relationships**:
- Belongs to Conversation (N:1)
- Has many QueuedAttachments (1:N)

**Validation Rules**:
- `retry_count` <= `max_retries`
- `message_content` non-empty
- `next_retry_at` must be future timestamp if present

### 5. UserPreferences

**Purpose**: Application settings and user configuration

**Fields**:
- `user_id: String` - User identifier (can be email or username)
- `global_hotkey: Option<String>` - Global hotkey combination
- `cache_policy: CachePolicy` - Default caching behavior
- `theme: Theme` - UI theme preference
- `window_settings: WindowSettings` - Window size/position preferences
- `notification_settings: NotificationSettings` - Notification preferences
- `quick_capture_enabled: bool` - Enable quick capture overlay
- `system_tray_enabled: bool` - Enable system tray integration
- `auto_start: bool` - Start application on system boot
- `analytics_enabled: bool` - Enable usage analytics
- `created_at: DateTime` - Preference creation time
- `updated_at: DateTime` - Last update time

**Validation Rules**:
- `user_id` non-empty string
- `global_hotkey` must be valid key combination format
- `updated_at` >= `created_at`

### 6. ServerConfiguration

**Purpose**: LibreChat server connection details

**Fields**:
- `id: String` - Configuration identifier
- `name: String` - Human-readable server name
- `base_url: String` - Server base URL (https://api.example.com)
- `auth_type: AuthType` - Enum: JWT, OAuth, LDAP
- `auth_token: Option<String>` - Encrypted JWT token
- `refresh_token: Option<String>` - Encrypted refresh token
- `token_expires_at: Option<DateTime>` - Token expiration
- `is_active: bool` - Currently selected server
- `is_secure: bool` - HTTPS connection
- `last_connected: Option<DateTime>` - Last successful connection
- `connection_status: ConnectionStatus` - Current connection state
- `api_version: String` - Supported API version
- `created_at: DateTime` - Configuration creation time

**Validation Rules**:
- `base_url` must be valid URL format
- `name` max length 100 characters
- Only one configuration can have `is_active = true`
- `auth_token` and `refresh_token` stored encrypted

### 7. DroppedFile

**Purpose**: Files dropped into application for processing

**Fields**:
- `id: String` - File identifier
- `conversation_id: Option<String>` - Associated conversation
- `original_name: String` - Original filename
- `file_path: String` - Local file path
- `mime_type: String` - File MIME type
- `size_bytes: u64` - File size
- `checksum: String` - SHA-256 checksum
- `upload_status: UploadStatus` - Processing state
- `server_file_id: Option<String>` - LibreChat file ID after upload
- `dropped_at: DateTime` - When file was dropped
- `processed_at: Option<DateTime>` - When processing completed
- `error_message: Option<String>` - Error if processing failed

**Validation Rules**:
- `original_name` non-empty string
- `size_bytes` > 0
- `mime_type` valid MIME format
- `file_path` must exist on filesystem

### 8. QuickCapture

**Purpose**: Temporary mini-conversation sessions

**Fields**:
- `id: String` - Session identifier
- `query: String` - User input query
- `response: Option<String>` - AI response
- `created_at: DateTime` - Session start time
- `completed_at: Option<DateTime>` - Session completion time
- `session_duration_ms: Option<u64>` - Session duration
- `converted_to_conversation: Option<String>` - If promoted to full conversation
- `server_id: String` - Server used for processing

**Validation Rules**:
- `query` non-empty string, max 1000 characters
- `completed_at` >= `created_at` if present
- `session_duration_ms` consistent with timestamps

## Enumerations

### SyncState
```rust
enum SyncState {
    Local,        // Only exists locally
    Synced,       // Synchronized with server
    Modified,     // Local changes pending sync
    Conflict,     // Sync conflict requires resolution
    Error,        // Sync failed with error
}
```

### CachePolicy
```rust
enum CachePolicy {
    Disabled,     // No caching
    Lightweight,  // 7 days / 100MB
    Balanced,     // 30 days / 500MB (default)
    Extended,     // 90 days / 2GB
}
```

### MessageRole
```rust
enum MessageRole {
    User,
    Assistant,
    System,
}
```

### ProcessingState
```rust
enum ProcessingState {
    Pending,      // Awaiting processing
    Processing,   // Currently being processed
    Complete,     // Successfully processed
    Failed,       // Processing failed
    Cancelled,    // Processing cancelled
}
```

### AuthType
```rust
enum AuthType {
    JWT,          // Username/password with JWT
    OAuth,        // OAuth provider
    LDAP,         // LDAP authentication
}
```

### ConnectionStatus
```rust
enum ConnectionStatus {
    Connected,    // Active connection
    Disconnected, // No connection
    Connecting,   // Connection in progress
    Error,        // Connection error
}
```

### UploadStatus
```rust
enum UploadStatus {
    Pending,      // Queued for upload
    Uploading,    // Upload in progress
    Completed,    // Successfully uploaded
    Failed,       // Upload failed
    Cancelled,    // Upload cancelled
}
```

## Database Schema (SQLite)

```sql
-- Conversations table
CREATE TABLE conversations (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    is_pinned BOOLEAN DEFAULT FALSE,
    is_starred BOOLEAN DEFAULT FALSE,
    server_id TEXT NOT NULL,
    sync_state TEXT NOT NULL,
    cache_policy TEXT NOT NULL,
    message_count INTEGER DEFAULT 0,
    last_message_preview TEXT,
    FOREIGN KEY (server_id) REFERENCES server_configurations(id)
);

-- Messages table
CREATE TABLE messages (
    id TEXT PRIMARY KEY,
    conversation_id TEXT NOT NULL,
    content TEXT NOT NULL,
    role TEXT NOT NULL,
    timestamp INTEGER NOT NULL,
    sync_state TEXT NOT NULL,
    metadata TEXT, -- JSON blob
    processing_state TEXT NOT NULL,
    FOREIGN KEY (conversation_id) REFERENCES conversations(id)
);

-- Additional tables follow similar patterns...
```

## Data Access Patterns

### Primary Queries
1. **Get Recent Conversations**: ORDER BY updated_at DESC LIMIT 10
2. **Search Messages**: Full-text search on content field
3. **Sync Pending Items**: WHERE sync_state = 'Modified' OR sync_state = 'Local'
4. **Cache Cleanup**: WHERE created_at < (NOW() - retention_period)
5. **Offline Queue**: WHERE retry_count < max_retries AND next_retry_at < NOW()

### Indexing Strategy
- Conversations: `(updated_at DESC)`, `(is_pinned, is_starred)`
- Messages: `(conversation_id, timestamp)`, `(content)` for FTS
- Cache: `(created_at)`, `(accessed_at)` for cleanup
- Queue: `(next_retry_at)`, `(created_at)`

## Performance Considerations

1. **Message Pagination**: Load messages in chunks of 50
2. **Cache Eviction**: Background task every 24 hours
3. **Search Optimization**: SQLite FTS5 for message content
4. **Encryption**: Encrypt/decrypt in background threads
5. **Sync Batching**: Batch sync operations to reduce API calls