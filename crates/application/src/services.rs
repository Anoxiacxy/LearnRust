use async_trait::async_trait;
use domain::{
    events::{DomainEvent, EventPublisher},
    market::{MarketData, Order},
    portfolio::{Portfolio, Position},
    strategy::{BacktestResult, StrategyConfig, TradingSignal},
};
use uuid::Uuid;

/// 市场监控服务
#[async_trait]
pub trait MarketMonitoringService: Send + Sync {
    /// 启动监控
    async fn start_monitoring(&self, symbols: Vec<String>) -> anyhow::Result<()>;
    
    /// 停止监控
    async fn stop_monitoring(&self) -> anyhow::Result<()>;
    
    /// 获取最新市场数据
    async fn get_latest_data(&self, symbol: &str) -> anyhow::Result<MarketData>;
    
    /// 设置价格警报
    async fn set_price_alert(&self, symbol: &str, price: f64) -> anyhow::Result<Uuid>;
}

/// 交易服务
#[async_trait]
pub trait TradingService: Send + Sync {
    /// 执行订单
    async fn execute_order(&self, order: Order) -> anyhow::Result<()>;
    
    /// 取消订单
    async fn cancel_order(&self, order_id: Uuid) -> anyhow::Result<()>;
    
    /// 获取订单状态
    async fn get_order_status(&self, order_id: Uuid) -> anyhow::Result<Order>;
    
    /// 根据信号自动交易
    async fn auto_trade(&self, signal: TradingSignal) -> anyhow::Result<()>;
}

/// 回测服务
#[async_trait]
pub trait BacktestService: Send + Sync {
    /// 运行回测
    async fn run_backtest(
        &self,
        strategy_id: Uuid,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
        initial_capital: f64,
    ) -> anyhow::Result<BacktestResult>;
    
    /// 优化策略参数
    async fn optimize_strategy(
        &self,
        strategy_config: StrategyConfig,
        optimization_params: serde_json::Value,
    ) -> anyhow::Result<StrategyConfig>;
}

/// 通知服务
#[async_trait]
pub trait NotificationService: Send + Sync {
    /// 发送交易警报
    async fn send_trade_alert(&self, message: String) -> anyhow::Result<()>;
    
    /// 发送风险警报
    async fn send_risk_alert(&self, message: String) -> anyhow::Result<()>;
    
    /// 发送日报
    async fn send_daily_report(&self, portfolio: &Portfolio) -> anyhow::Result<()>;
}

/// 风险管理服务
#[async_trait]
pub trait RiskManagementService: Send + Sync {
    /// 检查风险限制
    async fn check_risk_limits(&self, portfolio: &Portfolio, order: &Order) -> anyhow::Result<bool>;
    
    /// 计算止损价格
    async fn calculate_stop_loss(&self, position: &Position) -> anyhow::Result<f64>;
    
    /// 执行风险控制
    async fn execute_risk_control(&self, portfolio: &mut Portfolio) -> anyhow::Result<()>;
} 