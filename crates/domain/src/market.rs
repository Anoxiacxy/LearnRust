use crate::errors::DomainResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 市场数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketData {
    pub symbol: String,
    pub timestamp: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

/// 订单类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    Limit,
    StopLoss,
    TakeProfit,
}

/// 订单方向
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

/// 订单状态
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    Open,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
}

/// 订单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: uuid::Uuid,
    pub symbol: String,
    pub order_type: OrderType,
    pub side: OrderSide,
    pub quantity: f64,
    pub price: Option<f64>,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Order {
    pub fn new_market_order(symbol: String, side: OrderSide, quantity: f64) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            symbol,
            order_type: OrderType::Market,
            side,
            quantity,
            price: None,
            status: OrderStatus::Pending,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn new_limit_order(symbol: String, side: OrderSide, quantity: f64, price: f64) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            symbol,
            order_type: OrderType::Limit,
            side,
            quantity,
            price: Some(price),
            status: OrderStatus::Pending,
            created_at: now,
            updated_at: now,
        }
    }
}
