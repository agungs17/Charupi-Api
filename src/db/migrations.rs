use sqlx::SqlitePool;
use crate::error::AppResult;

pub async fn run_migrations(pool: &SqlitePool) -> AppResult<()> {
    // Migration 1: Skema tabel relasional inti
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS personas (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            avatar_file TEXT,
            chat_bg_file TEXT,
            personality TEXT,
            appearance TEXT,
            relationship TEXT,
            story_background TEXT NOT NULL DEFAULT '',
            first_message TEXT,
            extra_notes TEXT,
            message_examples TEXT,
            gender TEXT,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS user_persona (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT,
            appearance TEXT NOT NULL DEFAULT '',
            personality TEXT,
            gender TEXT,
            age TEXT NOT NULL DEFAULT '',
            extra_notes TEXT,
            is_default INTEGER NOT NULL DEFAULT 0,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS story_characters (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            persona_id INTEGER NOT NULL REFERENCES personas(id) ON DELETE CASCADE,
            name TEXT NOT NULL,
            nickname TEXT NOT NULL DEFAULT '',
            gender TEXT,
            age TEXT NOT NULL DEFAULT '',
            appearance TEXT NOT NULL DEFAULT '',
            personality TEXT NOT NULL DEFAULT '',
            relationship TEXT NOT NULL DEFAULT '',
            search_mode TEXT NOT NULL DEFAULT 'pinned',
            keywords TEXT NOT NULL DEFAULT '[]',
            is_active INTEGER NOT NULL DEFAULT 1,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS story_events (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            persona_id INTEGER NOT NULL REFERENCES personas(id) ON DELETE CASCADE,
            title TEXT NOT NULL,
            story TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'recent',
            search_mode TEXT NOT NULL DEFAULT 'pinned',
            keywords TEXT NOT NULL DEFAULT '[]',
            is_active INTEGER NOT NULL DEFAULT 1,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS story_event_characters (
            event_id INTEGER NOT NULL REFERENCES story_events(id) ON DELETE CASCADE,
            character_id INTEGER NOT NULL REFERENCES story_characters(id) ON DELETE CASCADE,
            PRIMARY KEY (event_id, character_id)
        );

        CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            persona_id INTEGER NOT NULL REFERENCES personas(id) ON DELETE CASCADE,
            role TEXT NOT NULL CHECK (role IN ('user', 'assistant', 'system')),
            content TEXT NOT NULL,
            chunk_id INTEGER,
            created_at INTEGER NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_messages_persona ON messages(persona_id, id DESC);

        CREATE TABLE IF NOT EXISTS omniroute_connections (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL DEFAULT '',
            base_url TEXT NOT NULL DEFAULT '',
            api_key TEXT NOT NULL DEFAULT '',
            model TEXT NOT NULL DEFAULT '',
            embedding_model TEXT NOT NULL DEFAULT '',
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS chunks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            persona_id INTEGER NOT NULL REFERENCES personas(id) ON DELETE CASCADE,
            first_msg_id INTEGER NOT NULL,
            last_msg_id INTEGER NOT NULL,
            text TEXT NOT NULL,
            embedding BLOB NOT NULL,
            model TEXT NOT NULL,
            created_at INTEGER NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_chunks_persona_last ON chunks(persona_id, last_msg_id);
        "#
    )
    .execute(pool)
    .await?;

    tracing::info!("Database schema migrations applied successfully");
    Ok(())
}
