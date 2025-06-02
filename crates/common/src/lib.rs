use serde_json;
use std::num::ParseIntError;
use thiserror::Error;

pub mod config;
pub mod di;
pub mod utils;

#[derive(Error, Debug)]
pub enum CommonError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("DI error: {0}")]
    DiError(#[from] di::DiError),
    #[error("Parse error: {0}")]
    ParseError(#[from] ParseIntError),
}

pub type Result<T> = std::result::Result<T, CommonError>;

pub mod types {
    use super::config::AppConfig;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Config {
        pub name: String,
        pub version: String,
    }

    impl From<AppConfig> for Config {
        fn from(app_config: AppConfig) -> Self {
            Self {
                name: app_config.name,
                version: app_config.version,
            }
        }
    }
}

// Re-export commonly used types
pub use config::AppConfig;
pub use di::Container;
