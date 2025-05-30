use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file: Option<PathBuf>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name: "my-app".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
                workers: num_cpus::get(),
            },
            database: DatabaseConfig {
                url: "postgres://localhost/db".to_string(),
                max_connections: 10,
                timeout_seconds: 30,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file: None,
            },
        }
    }
}

impl AppConfig {
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn from_env() -> crate::Result<Self> {
        let mut config = Self::default();

        if let Ok(host) = std::env::var("APP_HOST") {
            config.server.host = host;
        }
        if let Ok(port) = std::env::var("APP_PORT") {
            config.server.port = port.parse()?;
        }
        if let Ok(db_url) = std::env::var("DATABASE_URL") {
            config.database.url = db_url;
        }
        if let Ok(log_level) = std::env::var("RUST_LOG") {
            config.logging.level = log_level;
        }

        Ok(config)
    }
}
