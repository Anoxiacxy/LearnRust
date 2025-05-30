use crate::{DataProvider, DataSource, MarketData};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use csv::{Reader, Writer};
use log::info;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalMarketData {
    pub symbol: String,
    pub timestamp: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

pub struct LocalDataProvider {
    data_dir: String,
}

impl LocalDataProvider {
    pub fn new(data_dir: &str) -> Self {
        Self {
            data_dir: data_dir.to_string(),
        }
    }

    fn get_file_path(&self, symbol: &str) -> String {
        format!("{}/{}.csv", self.data_dir, symbol.to_lowercase())
    }

    fn read_data_from_file(&self, symbol: &str) -> Result<Vec<LocalMarketData>> {
        let file_path = self.get_file_path(symbol);
        if !Path::new(&file_path).exists() {
            return Ok(Vec::new());
        }

        let file = File::open(file_path)?;
        let mut reader = Reader::from_reader(file);
        let mut data = Vec::new();

        for result in reader.deserialize() {
            let record: LocalMarketData = result?;
            data.push(record);
        }

        Ok(data)
    }

    pub fn write_data_to_file(&self, symbol: &str, data: &[LocalMarketData]) -> Result<()> {
        let file_path = self.get_file_path(symbol);
        let file = File::create(file_path)?;
        let mut writer = Writer::from_writer(file);

        for record in data {
            writer.serialize(record)?;
        }

        writer.flush()?;
        Ok(())
    }

    fn convert_to_market_data(data: LocalMarketData) -> MarketData {
        MarketData {
            symbol: data.symbol,
            timestamp: DateTime::<Utc>::from_timestamp(data.timestamp, 0).unwrap(),
            open: data.open,
            high: data.high,
            low: data.low,
            close: data.close,
            volume: data.volume,
            source: DataSource::Local,
        }
    }
}

#[async_trait]
impl DataProvider for LocalDataProvider {
    async fn get_historical_data(
        &self,
        symbol: &str,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Result<Vec<MarketData>> {
        info!("Fetching historical data for {} from local storage", symbol);
        let local_data = self.read_data_from_file(symbol)?;

        let filtered_data: Vec<MarketData> = local_data
            .into_iter()
            .filter(|data| {
                let timestamp = DateTime::<Utc>::from_timestamp(data.timestamp, 0).unwrap();
                match (start_time, end_time) {
                    (Some(start), Some(end)) => timestamp >= start && timestamp <= end,
                    (Some(start), None) => timestamp >= start,
                    (None, Some(end)) => timestamp <= end,
                    (None, None) => true,
                }
            })
            .map(|data| Self::convert_to_market_data(data))
            .collect();

        Ok(filtered_data)
    }

    async fn get_latest_data(&self, symbol: &str) -> Result<MarketData> {
        info!("Fetching latest data for {} from local storage", symbol);
        let local_data = self.read_data_from_file(symbol)?;

        let latest_data = local_data
            .last()
            .ok_or_else(|| anyhow::anyhow!("No data available for {}", symbol))?;

        Ok(Self::convert_to_market_data(latest_data.clone()))
    }
}
