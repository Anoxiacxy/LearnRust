use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// API 服务器
pub struct ApiServer {
    // TODO: 添加应用服务依赖
}

impl ApiServer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn routes(&self) -> Router {
        Router::new()
            .route("/health", get(health))
            .route("/market/:symbol", get(get_market_data))
            .route("/orders", post(create_order))
            .route("/portfolio", get(get_portfolio))
    }

    pub async fn run(self, addr: &str) -> anyhow::Result<()> {
        let app = self.routes();
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}

// 健康检查
async fn health() -> &'static str {
    "OK"
}

// 获取市场数据
async fn get_market_data() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "not implemented"
    }))
}

// 创建订单
async fn create_order() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "not implemented"
    }))
}

// 获取投资组合
async fn get_portfolio() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "not implemented"
    }))
}
