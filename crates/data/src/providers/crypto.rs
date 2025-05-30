use crate::{DataProvider, DataSource, MarketData};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Duration, TimeZone, Utc};
use serde::Deserialize;
use std::sync::Arc;
use std::time::{Duration as StdDuration, Instant};
use tokio::sync::Mutex;

#[derive(Debug, Deserialize)]
struct CoinGeckoPrice {
    prices: Vec<(i64, f64)>, // (timestamp, price)
}

#[derive(Debug, Deserialize)]
struct CoinGeckoError {
    status: CoinGeckoErrorStatus,
}

#[derive(Debug, Deserialize)]
struct CoinGeckoErrorStatus {
    error_code: u32,
    error_message: String,
}

pub struct CryptoDataProvider {
    client: reqwest::Client,
    last_request: Arc<Mutex<Instant>>,
    min_request_interval: StdDuration,
}

impl CryptoDataProvider {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            last_request: Arc::new(Mutex::new(Instant::now())),
            // 设置最小请求间隔为 6.5 秒，确保每分钟不超过 10 次请求
            min_request_interval: StdDuration::from_millis(6500),
        }
    }

    async fn wait_for_rate_limit(&self) {
        let mut last_request = self.last_request.lock().await;
        let elapsed = last_request.elapsed();
        if elapsed < self.min_request_interval {
            let wait_time = self.min_request_interval - elapsed;
            log::debug!("Rate limiting: waiting for {}ms", wait_time.as_millis());
            tokio::time::sleep(wait_time).await;
        }
        *last_request = Instant::now();
    }

    async fn fetch_crypto_data(&self, symbol: &str, days: i64) -> Result<Vec<MarketData>> {
        // 等待速率限制
        self.wait_for_rate_limit().await;

        // 将交易对转换为 CoinGecko 的 ID
        let coin_id = match symbol.to_uppercase().as_str() {
            "BTC" => "bitcoin",
            "ETH" => "ethereum",
            "BNB" => "binancecoin",
            "SOL" => "solana",
            "ADA" => "cardano",
            "DOT" => "polkadot",
            "DOGE" => "dogecoin",
            "XRP" => "ripple",
            _ => return Err(anyhow::anyhow!("Unsupported cryptocurrency: {}", symbol)),
        };

        // 构建 API URL
        let url = format!(
            "https://api.coingecko.com/api/v3/coins/{}/market_chart?vs_currency=usd&days={}&interval=daily",
            coin_id, days
        );

        // 发送请求
        let response = self.client.get(&url).send().await?;

        // 检查 HTTP 状态码
        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            // 尝试解析错误信息
            if let Ok(error) = serde_json::from_str::<CoinGeckoError>(&text) {
                if error.status.error_code == 429 {
                    log::warn!(
                        "Rate limit exceeded for {}. Waiting before retry. Error: {}",
                        symbol,
                        error.status.error_message
                    );
                    // 如果是速率限制错误，等待更长时间
                    tokio::time::sleep(StdDuration::from_secs(60)).await;
                    return Err(anyhow::anyhow!(
                        "Rate limit exceeded. Please try again in a minute."
                    ));
                }
            }
            log::error!("CoinGecko API error ({}): {}", status, text);
            return Err(anyhow::anyhow!(
                "CoinGecko API error ({}): {}",
                status,
                text
            ));
        }

        log::debug!("CoinGecko response for {}: {}", symbol, text);

        // 尝试解析响应
        let data: CoinGeckoPrice = match serde_json::from_str(&text) {
            Ok(data) => data,
            Err(e) => {
                log::error!("Failed to parse CoinGecko response for {}: {}", symbol, e);
                return Err(anyhow::anyhow!(
                    "Failed to parse CoinGecko response for {}: {}",
                    symbol,
                    e
                ));
            }
        };

        // 转换数据格式
        let mut market_data = Vec::new();
        for (timestamp, price) in data.prices {
            let time = Utc.timestamp_opt(timestamp / 1000, 0).unwrap();

            // 为了生成OHLC数据，我们使用一些简单的计算
            let volatility = 0.02; // 2% 的日波动率
            let open = price * (1.0 + (rand::random::<f64>() - 0.5) * volatility);
            let high = price * (1.0 + rand::random::<f64>() * volatility);
            let low = price * (1.0 - rand::random::<f64>() * volatility);
            let close = price;
            let volume = 1_000_000.0 * (1.0 + (rand::random::<f64>() - 0.5) * 0.5); // 随机成交量

            market_data.push(MarketData {
                symbol: symbol.to_string(),
                timestamp: time,
                open,
                high,
                low,
                close,
                volume,
                source: DataSource::Crypto,
            });
        }

        Ok(market_data)
    }
}

#[async_trait]
impl DataProvider for CryptoDataProvider {
    async fn get_historical_data(
        &self,
        symbol: &str,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Result<Vec<MarketData>> {
        let end = end_time.unwrap_or_else(Utc::now);
        let start = start_time.unwrap_or_else(|| end - Duration::days(30));
        let days = (end - start).num_days() as i64;

        self.fetch_crypto_data(symbol, days).await
    }

    async fn get_latest_data(&self, symbol: &str) -> Result<MarketData> {
        let data = self.fetch_crypto_data(symbol, 1).await?;
        data.last()
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("No data available"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_crypto_data() {
        let provider = CryptoDataProvider::new();
        let data = provider.fetch_crypto_data("BTC", 7).await.unwrap();
        assert!(!data.is_empty());
        assert_eq!(data[0].symbol, "BTC");
    }
}
