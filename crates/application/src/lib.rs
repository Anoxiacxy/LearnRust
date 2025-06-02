pub mod analysis;
pub mod monitoring;
pub mod services;
pub mod trading;

// 重新导出核心服务
pub use services::{
    BacktestService, MarketMonitoringService, NotificationService, RiskManagementService,
    TradingService,
};

// 重新导出实现
pub use analysis::BacktestEngine;
pub use monitoring::MarketMonitor;
pub use trading::TradingEngine;
