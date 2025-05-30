use crate::{DataProvider, DataSource, MarketData};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use log::info;
use yahoo_finance_api as yahoo;

pub struct YahooFinanceProvider {
    client: yahoo::YahooConnector,
}

impl YahooFinanceProvider {
    pub fn new() -> Self {
        Self {
            client: yahoo::YahooConnector::new(),
        }
    }

    fn convert_timestamp(timestamp: u64) -> DateTime<Utc> {
        DateTime::<Utc>::from_timestamp(timestamp as i64, 0).unwrap()
    }

    fn convert_to_market_data(symbol: &str, quote: yahoo::Quote) -> MarketData {
        MarketData {
            symbol: symbol.to_string(),
            timestamp: Self::convert_timestamp(quote.timestamp),
            open: quote.open,
            high: quote.high,
            low: quote.low,
            close: quote.close,
            volume: quote.volume as f64,
            source: DataSource::Yahoo,
        }
    }
}

#[async_trait]
impl DataProvider for YahooFinanceProvider {
    async fn get_historical_data(
        &self,
        symbol: &str,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Result<Vec<MarketData>> {
        info!("Fetching historical data for {} from Yahoo Finance", symbol);

        let symbol = format!("{}.US", symbol);
        let start = start_time.unwrap_or_else(|| Utc::now() - chrono::Duration::days(30));
        let end = end_time.unwrap_or_else(Utc::now);

        let quotes = self
            .client
            .get_quote_history(&symbol, start, end)
            .await?
            .quotes()?;

        let market_data: Vec<MarketData> = quotes
            .into_iter()
            .map(|quote| Self::convert_to_market_data(&symbol, quote))
            .collect();

        Ok(market_data)
    }

    async fn get_latest_data(&self, symbol: &str) -> Result<MarketData> {
        info!("Fetching latest data for {} from Yahoo Finance", symbol);

        let symbol = format!("{}.US", symbol);
        let quotes = self
            .client
            .get_latest_quotes(&symbol, "1d")
            .await?
            .quotes()?;

        let latest_quote = quotes
            .last()
            .ok_or_else(|| anyhow::anyhow!("No data available for {}", symbol))?;

        Ok(Self::convert_to_market_data(&symbol, latest_quote.clone()))
    }
}
