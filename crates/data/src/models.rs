use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataSource {
    Local,
    Yahoo,
    YahooFinance,
    Crypto,
    Binance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataConfig {
    pub default_source: DataSource,
    pub cache_enabled: bool,
    pub cache_ttl: chrono::Duration,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketData {
    pub symbol: String,
    pub timestamp: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub source: DataSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataInterval {
    OneMinute,
    FiveMinutes,
    FifteenMinutes,
    ThirtyMinutes,
    OneHour,
    FourHours,
    OneDay,
    OneWeek,
    OneMonth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRequest {
    pub symbol: String,
    pub interval: DataInterval,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub source: DataSource,
}

#[async_trait]
pub trait DataProvider: Send + Sync {
    async fn get_historical_data(
        &self,
        symbol: &str,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Result<Vec<MarketData>>;

    async fn get_latest_data(&self, symbol: &str) -> Result<MarketData>;
}
