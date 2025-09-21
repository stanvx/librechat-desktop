use anyhow::Result;
use sqlx::SqlitePool;

const MIGRATIONS: &[&str] = &[
    // Server configurations table
    r#"
    CREATE TABLE IF NOT EXISTS server_configurations (
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL,
        base_url TEXT NOT NULL,
        auth_type TEXT NOT NULL,
        auth_token TEXT,
        refresh_token TEXT,
        token_expires_at INTEGER,
        is_active BOOLEAN DEFAULT 0,
        is_secure BOOLEAN DEFAULT 1,
        last_connected INTEGER,
        connection_status TEXT NOT NULL,
        api_version TEXT NOT NULL,
        created_at INTEGER NOT NULL
    );
    "#,
    r#"
    CREATE UNIQUE INDEX IF NOT EXISTS idx_server_config_active
        ON server_configurations (is_active)
        WHERE is_active = 1;
    "#,
    // Conversations
    r#"
    CREATE TABLE IF NOT EXISTS conversations (
        id TEXT PRIMARY KEY,
        title TEXT NOT NULL,
        created_at INTEGER NOT NULL,
        updated_at INTEGER NOT NULL,
        is_pinned BOOLEAN DEFAULT 0,
        is_starred BOOLEAN DEFAULT 0,
        server_id TEXT NOT NULL,
        sync_state TEXT NOT NULL,
        cache_policy TEXT NOT NULL,
        message_count INTEGER DEFAULT 0,
        last_message_preview TEXT,
        FOREIGN KEY (server_id) REFERENCES server_configurations(id)
            ON DELETE CASCADE
    );
    "#,
    r#"
    CREATE INDEX IF NOT EXISTS idx_conversations_updated_at
        ON conversations (updated_at DESC);
    "#,
    // Messages
    r#"
    CREATE TABLE IF NOT EXISTS messages (
        id TEXT PRIMARY KEY,
        conversation_id TEXT NOT NULL,
        content TEXT NOT NULL,
        role TEXT NOT NULL,
        timestamp INTEGER NOT NULL,
        sync_state TEXT NOT NULL,
        metadata TEXT,
        processing_state TEXT NOT NULL,
        FOREIGN KEY (conversation_id) REFERENCES conversations(id)
            ON DELETE CASCADE
    );
    "#,
    r#"
    CREATE INDEX IF NOT EXISTS idx_messages_conversation
        ON messages (conversation_id, timestamp DESC);
    "#,
    // Message attachments
    r#"
    CREATE TABLE IF NOT EXISTS message_attachments (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        message_id TEXT NOT NULL,
        file_id TEXT NOT NULL,
        filename TEXT NOT NULL,
        mime_type TEXT,
        size_bytes INTEGER,
        FOREIGN KEY (message_id) REFERENCES messages(id)
            ON DELETE CASCADE
    );
    "#,
    // Encrypted cache entries
    r#"
    CREATE TABLE IF NOT EXISTS encrypted_cache (
        cache_key TEXT PRIMARY KEY,
        encrypted_data BLOB NOT NULL,
        created_at INTEGER NOT NULL,
        accessed_at INTEGER NOT NULL,
        expires_at INTEGER,
        cache_type TEXT NOT NULL,
        size_bytes INTEGER NOT NULL,
        encryption_key_id TEXT NOT NULL
    );
    "#,
    // Message queue entries
    r#"
    CREATE TABLE IF NOT EXISTS message_queue (
        id TEXT PRIMARY KEY,
        conversation_id TEXT NOT NULL,
        message_content TEXT NOT NULL,
        created_at INTEGER NOT NULL,
        retry_count INTEGER NOT NULL,
        max_retries INTEGER NOT NULL,
        next_retry_at INTEGER,
        error_message TEXT,
        processing_state TEXT NOT NULL,
        FOREIGN KEY (conversation_id) REFERENCES conversations(id)
            ON DELETE CASCADE
    );
    "#,
    r#"
    CREATE TABLE IF NOT EXISTS message_queue_attachments (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        queue_id TEXT NOT NULL,
        file_path TEXT NOT NULL,
        mime_type TEXT,
        size_bytes INTEGER,
        FOREIGN KEY (queue_id) REFERENCES message_queue(id)
            ON DELETE CASCADE
    );
    "#,
    // User preferences
    r#"
    CREATE TABLE IF NOT EXISTS user_preferences (
        user_id TEXT PRIMARY KEY,
        global_hotkey TEXT,
        cache_policy TEXT NOT NULL,
        theme TEXT NOT NULL,
        window_settings TEXT NOT NULL,
        notification_settings TEXT NOT NULL,
        quick_capture_enabled BOOLEAN DEFAULT 1,
        system_tray_enabled BOOLEAN DEFAULT 1,
        auto_start BOOLEAN DEFAULT 0,
        analytics_enabled BOOLEAN DEFAULT 0,
        created_at INTEGER NOT NULL,
        updated_at INTEGER NOT NULL
    );
    "#,
    // Dropped files
    r#"
    CREATE TABLE IF NOT EXISTS dropped_files (
        id TEXT PRIMARY KEY,
        conversation_id TEXT,
        original_name TEXT NOT NULL,
        file_path TEXT NOT NULL,
        mime_type TEXT NOT NULL,
        size_bytes INTEGER NOT NULL,
        checksum TEXT NOT NULL,
        upload_status TEXT NOT NULL,
        server_file_id TEXT,
        dropped_at INTEGER NOT NULL,
        processed_at INTEGER,
        error_message TEXT,
        FOREIGN KEY (conversation_id) REFERENCES conversations(id)
            ON DELETE SET NULL
    );
    "#,
    // Quick capture sessions
    r#"
    CREATE TABLE IF NOT EXISTS quick_capture_sessions (
        id TEXT PRIMARY KEY,
        query TEXT NOT NULL,
        response TEXT,
        created_at INTEGER NOT NULL,
        completed_at INTEGER,
        session_duration_ms INTEGER,
        converted_to_conversation TEXT,
        server_id TEXT NOT NULL,
        FOREIGN KEY (server_id) REFERENCES server_configurations(id)
            ON DELETE CASCADE
    );
    "#,
    r#"
    CREATE INDEX IF NOT EXISTS idx_quick_capture_created
        ON quick_capture_sessions (created_at DESC);
    "#,
];

pub async fn apply_migrations(pool: &SqlitePool) -> Result<()> {
    for statement in MIGRATIONS {
        sqlx::query(statement).execute(pool).await?;
    }
    Ok(())
}
