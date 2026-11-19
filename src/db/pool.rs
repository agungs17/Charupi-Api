use std::{path::Path, str::FromStr, time::Duration};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    SqlitePool,
};
use crate::config::AppConfig;
use crate::error::AppResult;

pub async fn create_pool(config: &AppConfig) -> AppResult<SqlitePool> {
    // Pastikan folder parent database ada (misal data/)
    if let Some(path_str) = config.database_url.strip_prefix("sqlite://") {
        let clean_path = path_str.split('?').next().unwrap_or(path_str);
        if let Some(parent) = Path::new(clean_path).parent() {
            if !parent.as_os_str().is_empty() && !parent.exists() {
                tokio::fs::create_dir_all(parent).await.map_err(|e| {
                    crate::error::AppError::Internal(format!(
                        "Failed to create database directory: {}",
                        e
                    ))
                })?;
            }
        }
    }

    // High performance SQLite connection options
    let connect_opts = SqliteConnectOptions::from_str(&config.database_url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .foreign_keys(true)
        .busy_timeout(Duration::from_secs(10));

    let pool = SqlitePoolOptions::new()
        .max_connections(config.database_max_connections)
        .min_connections(config.database_min_connections)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(60))
        .connect_with(connect_opts)
        .await?;

    // Optimasi performa pragma pada SQLite pool
    sqlx::query("PRAGMA cache_size = -64000;") // 64MB memory cache
        .execute(&pool)
        .await?;
    sqlx::query("PRAGMA temp_store = MEMORY;")
        .execute(&pool)
        .await?;

    tracing::info!(
        url = %config.database_url,
        max_conns = config.database_max_connections,
        "SQLite connection pool initialized successfully with WAL mode"
    );

    Ok(pool)
}
