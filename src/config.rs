use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub environment: String,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let _ = dotenvy::dotenv();

        let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()?;
        let environment = std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());

        Ok(Self {
            host,
            port,
            environment,
        })
    }

    pub fn socket_addr(&self) -> anyhow::Result<SocketAddr> {
        let addr_str = format!("{}:{}", self.host, self.port);
        Ok(addr_str.parse()?)
    }
}
