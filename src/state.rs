use std::sync::Arc;
use sqlx::SqlitePool;
use reqwest::Client as HttpClient;
use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub http_client: HttpClient,
    pub config: Arc<AppConfig>,
}

impl AppState {
    pub fn new(db: SqlitePool, http_client: HttpClient, config: AppConfig) -> Self {
        Self {
            db,
            http_client,
            config: Arc::new(config),
        }
    }
}
