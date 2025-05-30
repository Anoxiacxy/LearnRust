use log::{error, info};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Common error: {0}")]
    Common(#[from] common::CommonError),
    #[error("Business logic error: {0}")]
    BusinessLogic(String),
}

pub type CoreResult<T> = std::result::Result<T, CoreError>;

pub struct CoreService {
    config: common::types::Config,
}

impl CoreService {
    pub fn new(config: common::types::Config) -> Self {
        Self { config }
    }

    pub async fn start(&self) -> CoreResult<()> {
        info!("Starting core service with config: {:?}", self.config);
        Ok(())
    }

    pub async fn process(&self, input: &str) -> CoreResult<String> {
        info!("Processing input: {}", input);
        Ok(format!("Processed: {}", input))
    }
}
