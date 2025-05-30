pub mod models;
pub mod providers;

// 重新导出数据提供者
pub use providers::crypto::CryptoDataProvider;

// 使用 models 模块中的类型定义
pub use models::{DataConfig, DataInterval, DataProvider, DataRequest, DataSource, MarketData};

pub struct DataService {
    provider: std::sync::Arc<dyn DataProvider>,
    config: DataConfig,
}

impl DataService {
    pub fn new(provider: std::sync::Arc<dyn DataProvider>, config: DataConfig) -> Self {
        Self { provider, config }
    }

    pub async fn get_historical_data(
        &self,
        symbol: &str,
        start_time: Option<chrono::DateTime<chrono::Utc>>,
        end_time: Option<chrono::DateTime<chrono::Utc>>,
    ) -> anyhow::Result<Vec<MarketData>> {
        self.provider
            .get_historical_data(symbol, start_time, end_time)
            .await
    }

    pub async fn get_latest_data(&self, symbol: &str) -> anyhow::Result<MarketData> {
        self.provider.get_latest_data(symbol).await
    }
}
