use anyhow::Result;
use chrono::Utc;
use data::models::{DataProvider, MarketData};
use data::providers::CryptoDataProvider;
use std::boxed::Box;
use std::thread;
use std::time::Duration;
use termion::clear;
use termion::cursor;

struct MarketMonitor {
    provider: Box<dyn DataProvider>,
    symbols: Vec<String>,
    update_interval: Duration,
}

impl MarketMonitor {
    fn new(provider: Box<dyn DataProvider>, symbols: Vec<String>, update_interval: Duration) -> Self {
        Self {
            provider,
            symbols,
            update_interval,
        }
    }

    async fn fetch_latest_data(&self, symbol: &str) -> Result<MarketData> {
        self.provider.get_latest_data(symbol).await
    }

    fn format_price(price: f64) -> String {
        format!("${:.2}", price)
    }

    fn format_change(open: f64, close: f64) -> String {
        let change = ((close - open) / open) * 100.0;
        let sign = if change >= 0.0 { "+" } else { "" };
        let color = if change >= 0.0 { "\x1b[32m" } else { "\x1b[31m" };
        format!("{}{}{}{:.2}%\x1b[0m", color, sign, change)
    }

    fn format_volume(volume: f64) -> String {
        if volume >= 1_000_000_000.0 {
            format!("{:.2}B", volume / 1_000_000_000.0)
        } else if volume >= 1_000_000.0 {
            format!("{:.2}M", volume / 1_000_000.0)
        } else if volume >= 1_000.0 {
            format!("{:.2}K", volume / 1_000.0)
        } else {
            format!("{:.2}", volume)
        }
    }

    async fn display_market_data(&self) -> Result<()> {
        println!("{}", clear::All);
        println!("{}", cursor::Goto(1, 1));
        println!("=== 加密货币实时行情 ===");
        println!("时间: {}", Utc::now().format("%Y-%m-%d %H:%M:%S UTC"));
        println!("\n{:<8} {:<12} {:<12} {:<12} {:<12} {:<12} {:<12}",
            "Symbol", "最新价", "24h涨跌", "开盘价", "最高价", "最低价", "成交量");
        println!("{}", "-".repeat(80));

        for symbol in &self.symbols {
            match self.fetch_latest_data(symbol).await {
                Ok(data) => {
                    println!("{:<8} {:<12} {:<12} {:<12} {:<12} {:<12} {:<12}",
                        data.symbol,
                        Self::format_price(data.close),
                        Self::format_change(data.open, data.close),
                        Self::format_price(data.open),
                        Self::format_price(data.high),
                        Self::format_price(data.low),
                        Self::format_volume(data.volume)
                    );
                }
                Err(e) => {
                    println!("{:<8} Error: {}", symbol, e);
                }
            }
        }

        println!("\n按 Ctrl+C 退出");
        Ok(())
    }

    async fn run(&self) -> Result<()> {
        loop {
            self.display_market_data().await?;
            thread::sleep(self.update_interval);
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // 设置要监控的交易对
    let symbols = vec![
        "BTC".to_string(),
        "ETH".to_string(),
        "BNB".to_string(),
        "SOL".to_string(),
        "ADA".to_string(),
    ];

    // 创建数据提供者
    let provider: Box<dyn DataProvider> = Box::new(CryptoDataProvider::new());

    // 创建市场监控器（每5秒更新一次）
    let monitor = MarketMonitor::new(
        provider,
        symbols,
        Duration::from_secs(5)
    );

    // 运行监控器
    monitor.run().await?;

    Ok(())
} 