pub mod errors;
pub mod events;
pub mod market;
pub mod portfolio;
pub mod strategy;

// 重新导出核心类型
pub use errors::{DomainError, DomainResult};
pub use events::{DomainEvent, EventPublisher, EventSubscriber};
