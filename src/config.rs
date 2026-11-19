use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub environment: String,
    pub database_url: String,
    pub database_max_connections: u32,
    pub database_min_connections: u32,
    pub omniroute_base_url: String,
    pub omniroute_api_key: Option<String>,
    pub omniroute_default_model: String,
    pub omniroute_embedding_model: String,
    pub omniroute_timeout_secs: u64,
    pub vector_dimension: usize,
    pub vector_top_k: usize,
    pub vector_cosine_threshold: f32,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let _ = dotenvy::dotenv();

        let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()?;
        let environment = std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite://data/charupi.db?mode=rwc".to_string());

        let database_max_connections = std::env::var("DATABASE_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "16".to_string())
            .parse::<u32>()?;
        let database_min_connections = std::env::var("DATABASE_MIN_CONNECTIONS")
            .unwrap_or_else(|_| "2".to_string())
            .parse::<u32>()?;

        let omniroute_base_url = std::env::var("OMNIROUTE_BASE_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:20128/v1".to_string())
            .trim_end_matches('/')
            .to_string();

        let omniroute_api_key = std::env::var("OMNIROUTE_API_KEY")
            .ok()
            .filter(|k| !k.trim().is_empty());

        let omniroute_default_model = std::env::var("OMNIROUTE_DEFAULT_MODEL")
            .unwrap_or_else(|_| "gpt-4o-mini".to_string());

        let omniroute_embedding_model = std::env::var("OMNIROUTE_EMBEDDING_MODEL")
            .unwrap_or_else(|_| "text-embedding-3-small".to_string());

        let omniroute_timeout_secs = std::env::var("OMNIROUTE_TIMEOUT_SECS")
            .unwrap_or_else(|_| "120".to_string())
            .parse::<u64>()?;

        let vector_dimension = std::env::var("VECTOR_DIMENSION")
            .unwrap_or_else(|_| "768".to_string())
            .parse::<usize>()?;

        let vector_top_k = std::env::var("VECTOR_TOP_K")
            .unwrap_or_else(|_| "5".to_string())
            .parse::<usize>()?;

        let vector_cosine_threshold = std::env::var("VECTOR_COSINE_THRESHOLD")
            .unwrap_or_else(|_| "0.55".to_string())
            .parse::<f32>()?;

        Ok(Self {
            host,
            port,
            environment,
            database_url,
            database_max_connections,
            database_min_connections,
            omniroute_base_url,
            omniroute_api_key,
            omniroute_default_model,
            omniroute_embedding_model,
            omniroute_timeout_secs,
            vector_dimension,
            vector_top_k,
            vector_cosine_threshold,
        })
    }

    pub fn socket_addr(&self) -> anyhow::Result<SocketAddr> {
        let addr_str = format!("{}:{}", self.host, self.port);
        Ok(addr_str.parse()?)
    }
}
