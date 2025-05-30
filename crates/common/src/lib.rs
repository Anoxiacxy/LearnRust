use serde_json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommonError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, CommonError>;

pub mod types {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Config {
        pub name: String,
        pub version: String,
    }
}

pub mod utils {
    use log::info;

    pub fn init_logging() {
        env_logger::init();
        info!("Logging initialized");
    }
}
