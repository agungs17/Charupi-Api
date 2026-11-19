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

    /// Memeriksa apakah environment aktif adalah development
    pub fn is_development(&self) -> bool {
        self.environment.eq_ignore_ascii_case("development")
    }

    /// Memeriksa apakah environment aktif adalah production
    pub fn is_production(&self) -> bool {
        self.environment.eq_ignore_ascii_case("production")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_values() {
        let dev_config = AppConfig {
            host: "0.0.0.0".into(),
            port: 8080,
            environment: "development".into(),
        };
        assert!(dev_config.is_development());
        assert!(!dev_config.is_production());

        let prod_config = AppConfig {
            host: "0.0.0.0".into(),
            port: 8080,
            environment: "production".into(),
        };
        assert!(prod_config.is_production());
        assert!(!prod_config.is_development());

        // "dev" tidak boleh diakui sebagai development
        let dev_short = AppConfig {
            host: "0.0.0.0".into(),
            port: 8080,
            environment: "dev".into(),
        };
        assert!(!dev_short.is_development());
        assert!(!dev_short.is_production());
    }
}
