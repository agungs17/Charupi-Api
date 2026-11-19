use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub name: String,
    pub url: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5432,
            user: "charupi".to_string(),
            password: "charupi_password".to_string(),
            name: "charupi_db".to_string(),
            url: "postgres://charupi:charupi_password@localhost:5432/charupi_db".to_string(),
        }
    }
}

impl DatabaseConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let host = std::env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = std::env::var("DB_PORT")
            .unwrap_or_else(|_| "5432".to_string())
            .parse::<u16>()?;
        let user = std::env::var("DB_USER").unwrap_or_else(|_| "charupi".to_string());
        let password = std::env::var("DB_PASSWORD").unwrap_or_else(|_| "charupi_password".to_string());
        let name = std::env::var("DB_NAME").unwrap_or_else(|_| "charupi_db".to_string());
        let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            format!("postgres://{}:{}@{}:{}/{}", user, password, host, port, name)
        });

        Ok(Self {
            host,
            port,
            user,
            password,
            name,
            url,
        })
    }
}

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub environment: String,
    pub database: DatabaseConfig,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let _ = dotenvy::dotenv();

        let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()?;
        let environment = std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
        let database = DatabaseConfig::from_env()?;

        Ok(Self {
            host,
            port,
            environment,
            database,
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
            database: DatabaseConfig::default(),
        };
        assert!(dev_config.is_development());
        assert!(!dev_config.is_production());

        let prod_config = AppConfig {
            host: "0.0.0.0".into(),
            port: 8080,
            environment: "production".into(),
            database: DatabaseConfig::default(),
        };
        assert!(prod_config.is_production());
        assert!(!prod_config.is_development());

        // "dev" tidak boleh diakui sebagai development
        let dev_short = AppConfig {
            host: "0.0.0.0".into(),
            port: 8080,
            environment: "dev".into(),
            database: DatabaseConfig::default(),
        };
        assert!(!dev_short.is_development());
        assert!(!dev_short.is_production());
    }

    #[test]
    fn test_database_config_default_and_url() {
        let db = DatabaseConfig::default();
        assert_eq!(db.host, "localhost");
        assert_eq!(db.port, 5432);
        assert_eq!(db.user, "charupi");
        assert_eq!(db.password, "charupi_password");
        assert_eq!(db.name, "charupi_db");
        assert_eq!(
            db.url,
            "postgres://charupi:charupi_password@localhost:5432/charupi_db"
        );
    }
}
