use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use clap::{Parser, Subcommand};
use data::models::{DataProvider, MarketData};
use data::providers::CryptoDataProvider;
use log;
use plot::{ChartPlotter, ChartStyle};
use std::boxed::Box;
use std::thread;
use std::time::{Duration as StdDuration, Instant};
use termion::clear;
use termion::cursor;

#[derive(Parser)]
#[command(
    name = "quant-cli",
    version,
    about = "量化交易平台 CLI",
    author = "Your Name"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 绘制行情图表
    Plot {
        #[arg(long, default_value = "BTC")]
        symbol: String,
    },
    /// 实时行情监控
    Realtime {
        #[arg(long, default_value = "BTC,ETH,BNB,SOL,ADA")]
        symbols: String,
        #[arg(long, default_value_t = 300)] // 默认5分钟更新一次
        interval: u64,
    },
}

struct MarketMonitor {
    provider: Box<dyn DataProvider>,
    symbols: Vec<String>,
    update_interval: StdDuration,
    last_update: Instant,
    market_data: Vec<MarketData>,
}

impl MarketMonitor {
    fn new(
        provider: Box<dyn DataProvider>,
        symbols: Vec<String>,
        update_interval: StdDuration,
    ) -> Self {
        Self {
            provider,
            symbols,
            update_interval,
            last_update: Instant::now(),
            market_data: Vec::new(),
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
        let color = if change >= 0.0 {
            "\x1b[32m"
        } else {
            "\x1b[31m"
        };
        format!("{}{}{:.2}%\x1b[0m", color, sign, change)
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

    fn format_countdown(&self) -> String {
        let elapsed = self.last_update.elapsed();
        if elapsed >= self.update_interval {
            "更新中...".to_string()
        } else {
            let remaining = self.update_interval - elapsed;
            format!("{}秒后更新 (API限制: 每分钟10次请求)", remaining.as_secs())
        }
    }

    async fn update_market_data(&mut self) -> Result<()> {
        let mut new_data = Vec::new();
        for symbol in &self.symbols {
            match self.fetch_latest_data(symbol).await {
                Ok(data) => new_data.push(data),
                Err(e) => {
                    log::error!("Failed to fetch data for {}: {}", symbol, e);
                }
            }
        }
        if !new_data.is_empty() {
            self.market_data = new_data;
            self.last_update = Instant::now();
        }
        Ok(())
    }

    async fn display_market_data(&self) -> Result<()> {
        print!("\x1B[2J\x1B[1;1H"); // 清屏并移动光标到开始位置
        println!("=== 加密货币实时行情 ===");
        println!(
            "时间: {} ({} UTC)",
            Utc::now().format("%Y-%m-%d %H:%M:%S"),
            self.format_countdown()
        );
        println!("注意: CoinGecko API 免费版限制每分钟10次请求，建议更新间隔不小于5分钟");
        println!(
            "\n{:<8} {:<12} {:<12} {:<12} {:<12} {:<12} {:<12}",
            "Symbol", "最新价", "24h涨跌", "开盘价", "最高价", "最低价", "成交量"
        );
        println!("{}", "-".repeat(80));

        for data in &self.market_data {
            println!(
                "{:<8} {:<12} {:<12} {:<12} {:<12} {:<12} {:<12}",
                data.symbol,
                Self::format_price(data.close),
                Self::format_change(data.open, data.close),
                Self::format_price(data.open),
                Self::format_price(data.high),
                Self::format_price(data.low),
                Self::format_volume(data.volume)
            );
        }

        println!("\n按 Ctrl+C 退出");
        Ok(())
    }

    async fn run(&mut self) -> Result<()> {
        // 首次更新数据
        self.update_market_data().await?;

        loop {
            // 显示当前数据
            self.display_market_data().await?;

            // 每秒更新一次显示
            thread::sleep(StdDuration::from_secs(1));

            // 如果到达更新间隔，则更新数据
            if self.last_update.elapsed() >= self.update_interval {
                self.update_market_data().await?;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Plot { symbol } => {
            // 获取最近30天行情
            let provider: Box<dyn DataProvider> = Box::new(CryptoDataProvider::new());
            let data = provider.get_historical_data(&symbol, None, None).await?;
            let style = ChartStyle::default();
            let plotter = ChartPlotter::new(data)
                .with_style(style)
                .with_ma_periods(vec![5, 10, 20]);
            plotter.save_to_file("market_chart.png")?;
            println!("已生成 market_chart.png");
        }
        Commands::Realtime { symbols, interval } => {
            let symbol_list: Vec<String> = symbols
                .split(',')
                .map(|s| s.trim().to_uppercase())
                .collect();
            let provider: Box<dyn DataProvider> = Box::new(CryptoDataProvider::new());
            let mut monitor =
                MarketMonitor::new(provider, symbol_list, StdDuration::from_secs(interval));
            monitor.run().await?;
        }
    }
    Ok(())
}
