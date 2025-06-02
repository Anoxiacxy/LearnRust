use chrono::{DateTime, Utc};
use data::models::MarketData;

/// 计算价格变化百分比
pub fn calculate_price_change(data: &[MarketData]) -> Option<f64> {
    if data.len() < 2 {
        return None;
    }

    let first = data.first()?.close;
    let last = data.last()?.close;
    Some((last - first) / first * 100.0)
}

/// 格式化时间戳
pub fn format_timestamp(timestamp: DateTime<Utc>) -> String {
    timestamp.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// 检查数据是否有效
pub fn is_valid_data(data: &MarketData) -> bool {
    data.close > 0.0 && data.volume >= 0.0
}
