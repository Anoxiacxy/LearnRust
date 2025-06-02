use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Invalid market data: {0}")]
    InvalidMarketData(String),

    #[error("Strategy error: {0}")]
    StrategyError(String),

    #[error("Portfolio error: {0}")]
    PortfolioError(String),

    #[error("Risk limit exceeded: {0}")]
    RiskLimitExceeded(String),

    #[error("Insufficient funds: {0}")]
    InsufficientFunds(String),

    #[error("Invalid order: {0}")]
    InvalidOrder(String),
}

pub type DomainResult<T> = Result<T, DomainError>;
