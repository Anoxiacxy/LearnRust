use crate::{DataProvider, DataSource, MarketData};
use anyhow::Result;
use async_trait::async_trait;
use binance::{
    api::*,
    market::*,
    model::{KlineSummaries, KlineSummary},
};
use chrono::{DateTime, Duration, TimeZone, Utc};
use std::sync::Arc;

pub struct BinanceDataProvider {
    market: Arc<Market>,
}

impl BinanceDataProvider {
    pub fn new() -> Self {
        Self {
            market: Arc::new(Market::new(None, None)),
        }
    }

    fn convert_symbol(symbol: &str) -> String {
        format!("{}USDT", symbol.to_uppercase())
    }

    fn convert_interval(days: i64) -> &'static str {
        if days <= 1 {
            "1m"
        } else if days <= 7 {
            "15m"
        } else if days <= 30 {
            "1h"
        } else {
            "1d"
        }
    }

    fn fetch_binance_data(&self, symbol: &str, days: i64) -> Result<Vec<MarketData>> {
        let binance_symbol = Self::convert_symbol(symbol);
        let interval = Self::convert_interval(days);

        let end_time = Utc::now();
        let start_time = end_time - Duration::days(days);

        // 将时间戳转换为 u64
        let start_ms = start_time.timestamp_millis() as u64;
        let end_ms = end_time.timestamp_millis() as u64;

        // 获取K线数据
        let klines_enum = self
            .market
            .get_klines(
                &binance_symbol,
                interval,
                None,
                Some(start_ms),
                Some(end_ms),
            )
            .map_err(|e| anyhow::anyhow!("Binance API error: {}", e))?;

        let klines: Vec<KlineSummary> = match klines_enum {
            KlineSummaries::AllKlineSummaries(k) => k,
            _ => vec![],
        };

        let market_data = klines
            .iter()
            .map(|kline| MarketData {
                symbol: symbol.to_string(),
                timestamp: Utc.timestamp_millis_opt(kline.open_time as i64).unwrap(),
                open: kline.open.parse().unwrap_or(0.0),
                high: kline.high.parse().unwrap_or(0.0),
                low: kline.low.parse().unwrap_or(0.0),
                close: kline.close.parse().unwrap_or(0.0),
                volume: kline.volume.parse().unwrap_or(0.0),
                source: DataSource::Binance,
            })
            .collect();

        Ok(market_data)
    }
}

#[async_trait]
impl DataProvider for BinanceDataProvider {
    async fn get_historical_data(
        &self,
        symbol: &str,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Result<Vec<MarketData>> {
        let end = end_time.unwrap_or_else(Utc::now);
        let start = start_time.unwrap_or_else(|| end - Duration::days(30));
        let days = (end - start).num_days() as i64;

        // 克隆需要的值以解决生命周期问题
        let symbol = symbol.to_string();
        let market = Arc::clone(&self.market);

        tokio::task::spawn_blocking(move || {
            let provider = BinanceDataProvider { market };
            provider.fetch_binance_data(&symbol, days)
        })
        .await?
    }

    async fn get_latest_data(&self, symbol: &str) -> Result<MarketData> {
        let symbol = symbol.to_string();
        let market = Arc::clone(&self.market);

        let data = tokio::task::spawn_blocking(move || {
            let provider = BinanceDataProvider { market };
            provider.fetch_binance_data(&symbol, 1)
        })
        .await??;

        data.last()
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("No data available"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_binance_data() {
        let provider = BinanceDataProvider::new();
        let data = provider.fetch_binance_data("BTC", 1).unwrap();
        assert!(!data.is_empty());
        assert_eq!(data[0].symbol, "BTC");
    }
}
