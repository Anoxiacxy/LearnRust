use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// 领域事件基础trait
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainEvent {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub event_type: EventType,
    pub payload: EventPayload,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum EventType {
    // Market events
    MarketDataUpdated,
    PriceAlert,

    // Trading events
    OrderPlaced,
    OrderFilled,
    OrderCancelled,

    // Portfolio events
    PositionOpened,
    PositionClosed,
    PortfolioRebalanced,

    // Strategy events
    SignalGenerated,
    StrategyActivated,
    StrategyDeactivated,

    // Risk events
    RiskLimitTriggered,
    StopLossTriggered,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventPayload {
    MarketData {
        symbol: String,
        price: f64,
    },
    Order {
        id: Uuid,
        symbol: String,
        quantity: f64,
        price: f64,
    },
    Position {
        id: Uuid,
        symbol: String,
        quantity: f64,
    },
    Signal {
        strategy_id: Uuid,
        symbol: String,
        action: String,
    },
    RiskAlert {
        message: String,
        severity: String,
    },
}

/// 事件发布者接口
#[async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish(&self, event: DomainEvent) -> anyhow::Result<()>;
}

/// 事件订阅者接口
#[async_trait]
pub trait EventSubscriber: Send + Sync {
    async fn subscribe(&self, event_types: Vec<EventType>) -> anyhow::Result<()>;
    async fn handle_event(&self, event: DomainEvent) -> anyhow::Result<()>;
}
