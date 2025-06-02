use anyhow::Result;
use data::models::{DataProvider, MarketData};
use log::{info, warn};
use plot::{ChartPlotter, ChartStyle};
use std::boxed::Box;
use std::time::Duration;
use tokio::time;

pub struct MarketMonitor {
    provider: Box<dyn DataProvider>,
    symbols: Vec<String>,
    interval: Duration,
}

impl MarketMonitor {
    pub fn new(provider: Box<dyn DataProvider>, symbols: Vec<String>, interval: Duration) -> Self {
        Self {
            provider,
            symbols,
            interval,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        info!("开始监控市场数据...");
        info!("监控的币种: {:?}", self.symbols);
        info!("更新间隔: {} 秒", self.interval.as_secs());

        let mut interval = time::interval(self.interval);

        loop {
            interval.tick().await;
            for symbol in &self.symbols {
                match self.provider.get_latest_data(symbol).await {
                    Ok(data) => {
                        self.handle_market_data(&data).await?;
                    }
                    Err(e) => {
                        warn!("获取 {} 数据失败: {}", symbol, e);
                    }
                }
            }
        }
    }

    async fn handle_market_data(&self, data: &MarketData) -> Result<()> {
        info!(
            "{} - 最新价格: {:.2}, 24h成交量: {:.2}",
            data.symbol, data.close, data.volume
        );

        // 获取历史数据并绘制图表
        let historical_data = self
            .provider
            .get_historical_data(&data.symbol, None, None)
            .await?;

        let style = ChartStyle::default();
        let plotter = ChartPlotter::new(historical_data)
            .with_style(style)
            .with_ma_periods(vec![5, 10, 20]);

        let filename = format!("{}_chart.png", data.symbol.to_lowercase());
        plotter.save_to_file(&filename)?;
        info!("已生成图表: {}", filename);

        Ok(())
    }
}
