use anyhow::Result;
use app_core::{Cli, Commands, MarketMonitor};
use clap::Parser;
use data::models::DataProvider;
use data::providers::CryptoDataProvider;
use plot::{ChartPlotter, ChartStyle};
use std::boxed::Box;
use std::time::Duration as StdDuration;

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
