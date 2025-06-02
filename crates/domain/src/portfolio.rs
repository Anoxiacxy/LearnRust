use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use crate::market::{Order, OrderSide};
use crate::errors::{DomainError, DomainResult};

/// 持仓
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub id: Uuid,
    pub symbol: String,
    pub quantity: f64,
    pub entry_price: f64,
    pub current_price: f64,
    pub side: OrderSide,
    pub opened_at: DateTime<Utc>,
    pub stop_loss: Option<f64>,
    pub take_profit: Option<f64>,
}

impl Position {
    pub fn unrealized_pnl(&self) -> f64 {
        match self.side {
            OrderSide::Buy => (self.current_price - self.entry_price) * self.quantity,
            OrderSide::Sell => (self.entry_price - self.current_price) * self.quantity,
        }
    }
    
    pub fn unrealized_pnl_percentage(&self) -> f64 {
        match self.side {
            OrderSide::Buy => ((self.current_price - self.entry_price) / self.entry_price) * 100.0,
            OrderSide::Sell => ((self.entry_price - self.current_price) / self.entry_price) * 100.0,
        }
    }
}

/// 投资组合
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub id: Uuid,
    pub name: String,
    pub cash_balance: f64,
    pub positions: HashMap<String, Position>,
    pub closed_trades: Vec<ClosedTrade>,
    pub performance: PerformanceMetrics,
}

/// 已关闭交易
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClosedTrade {
    pub id: Uuid,
    pub symbol: String,
    pub quantity: f64,
    pub entry_price: f64,
    pub exit_price: f64,
    pub side: OrderSide,
    pub pnl: f64,
    pub opened_at: DateTime<Utc>,
    pub closed_at: DateTime<Utc>,
}

/// 绩效指标
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceMetrics {
    pub total_return: f64,
    pub total_return_percentage: f64,
    pub win_rate: f64,
    pub profit_factor: f64,
    pub max_drawdown: f64,
    pub sharpe_ratio: f64,
    pub total_trades: u32,
    pub winning_trades: u32,
    pub losing_trades: u32,
}

/// 投资组合管理器trait
#[async_trait]
pub trait PortfolioManager: Send + Sync {
    /// 开仓
    async fn open_position(&mut self, order: Order) -> DomainResult<Position>;
    
    /// 平仓
    async fn close_position(&mut self, symbol: &str) -> DomainResult<ClosedTrade>;
    
    /// 更新持仓价格
    async fn update_position_price(&mut self, symbol: &str, price: f64) -> DomainResult<()>;
    
    /// 获取总资产价值
    fn total_value(&self) -> f64;
    
    /// 获取可用资金
    fn available_cash(&self) -> f64;
    
    /// 计算风险指标
    fn calculate_risk_metrics(&self) -> RiskMetrics;
}

/// 风险指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskMetrics {
    pub var_95: f64, // 95% Value at Risk
    pub position_exposure: f64,
    pub leverage: f64,
    pub correlation_risk: f64,
}

/// 资金管理器trait
#[async_trait]
pub trait MoneyManager: Send + Sync {
    /// 计算仓位大小
    async fn calculate_position_size(
        &self,
        portfolio: &Portfolio,
        symbol: &str,
        entry_price: f64,
        stop_loss: f64,
    ) -> DomainResult<f64>;
    
    /// 验证风险限制
    async fn validate_risk_limits(
        &self,
        portfolio: &Portfolio,
        order: &Order,
    ) -> DomainResult<()>;
} 