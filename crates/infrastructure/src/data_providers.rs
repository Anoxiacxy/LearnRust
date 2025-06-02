use data::providers::CryptoDataProvider;

/// 数据提供者适配器
pub struct DataProviderAdapter {
    crypto_provider: CryptoDataProvider,
}

impl DataProviderAdapter {
    pub fn new() -> Self {
        Self {
            crypto_provider: CryptoDataProvider::new(),
        }
    }

    // TODO: 实现领域模型与基础设施层的适配
}
