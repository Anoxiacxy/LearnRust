use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::market::{MarketData, OrderSide};
use crate::errors::DomainResult;

/// 交易信号
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingSignal {
    pub id: Uuid,
    pub strategy_id: Uuid,
    pub symbol: String,
    pub action: SignalAction,
    pub strength: f64, // 0.0 到 1.0
    pub timestamp: DateTime<Utc>,
    pub metadata: SignalMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignalAction {
    Buy,
    Sell,
    Hold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalMetadata {
    pub indicators: Vec<IndicatorValue>,
    pub confidence: f64,
    pub risk_level: RiskLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorValue {
    pub name: String,
    pub value: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

/// 策略配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyConfig {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
    pub risk_parameters: RiskParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskParameters {
    pub max_position_size: f64,
    pub stop_loss_percentage: f64,
    pub take_profit_percentage: f64,
    pub max_daily_trades: u32,
}

/// 回测结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestResult {
    pub strategy_id: Uuid,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub initial_capital: f64,
    pub final_capital: f64,
    pub total_trades: u32,
    pub winning_trades: u32,
    pub losing_trades: u32,
    pub max_drawdown: f64,
    pub sharpe_ratio: f64,
    pub profit_factor: f64,
}

/// 交易策略trait
#[async_trait]
pub trait TradingStrategy: Send + Sync {
    /// 获取策略ID
    fn id(&self) -> Uuid;
    
    /// 获取策略名称
    fn name(&self) -> &str;
    
    /// 分析市场数据并生成交易信号
    async fn analyze(&self, data: &[MarketData]) -> DomainResult<Option<TradingSignal>>;
    
    /// 验证策略参数
    fn validate_parameters(&self) -> DomainResult<()>;
    
    /// 获取策略配置
    fn config(&self) -> &StrategyConfig;
}

/// 策略工厂trait
#[async_trait]
pub trait StrategyFactory: Send + Sync {
    /// 创建策略实例
    async fn create_strategy(&self, config: StrategyConfig) -> DomainResult<Box<dyn TradingStrategy>>;
    
    /// 获取支持的策略类型
    fn supported_strategies(&self) -> Vec<String>;
} 