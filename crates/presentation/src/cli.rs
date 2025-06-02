use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "quant-cli",
    version,
    about = "量化交易平台 CLI",
    author = "Your Name"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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
    /// 运行回测
    Backtest {
        #[arg(long)]
        strategy: String,
        #[arg(long)]
        start_date: String,
        #[arg(long)]
        end_date: String,
        #[arg(long, default_value_t = 10000.0)]
        initial_capital: f64,
    },
    /// 启动 API 服务器
    Server {
        #[arg(long, default_value = "127.0.0.1:8080")]
        addr: String,
    },
}
