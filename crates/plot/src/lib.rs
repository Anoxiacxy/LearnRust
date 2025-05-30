use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use data::MarketData;
use plotters::prelude::*;
use std::path::Path;

/// 图表样式配置
#[derive(Clone, Debug)]
pub struct ChartStyle {
    pub width: u32,
    pub height: u32,
    pub background_color: RGBColor,
    pub text_color: RGBColor,
    pub grid_color: RGBColor,
    pub bull_color: RGBColor,                // 上涨颜色
    pub bear_color: RGBColor,                // 下跌颜色
    pub ma_colors: Vec<RGBColor>,            // 均线颜色
    pub signal_colors: (RGBColor, RGBColor), // (买入信号颜色, 卖出信号颜色)
}

impl Default for ChartStyle {
    fn default() -> Self {
        Self {
            width: 1200,
            height: 800,
            background_color: RGBColor(240, 240, 240),
            text_color: RGBColor(0, 0, 0),
            grid_color: RGBColor(200, 200, 200),
            bull_color: RGBColor(0, 150, 0), // 绿色
            bear_color: RGBColor(150, 0, 0), // 红色
            ma_colors: vec![
                RGBColor(0, 0, 255),   // 蓝色
                RGBColor(255, 0, 255), // 紫色
                RGBColor(255, 165, 0), // 橙色
            ],
            signal_colors: (
                RGBColor(0, 100, 0), // 深绿色
                RGBColor(100, 0, 0), // 深红色
            ),
        }
    }
}

/// 图表绘制器
pub struct ChartPlotter {
    style: ChartStyle,
    data: Vec<MarketData>,
    ma_periods: Vec<usize>,                     // 均线周期
    signals: Vec<(DateTime<Utc>, f64, String)>, // (时间, 价格, 信号类型)
}

impl ChartPlotter {
    pub fn new(data: Vec<MarketData>) -> Self {
        Self {
            style: ChartStyle::default(),
            data,
            ma_periods: vec![5, 10, 20], // 默认均线周期
            signals: Vec::new(),
        }
    }

    pub fn with_style(mut self, style: ChartStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_ma_periods(mut self, periods: Vec<usize>) -> Self {
        self.ma_periods = periods;
        self
    }

    pub fn add_signal(&mut self, time: DateTime<Utc>, price: f64, signal_type: &str) {
        self.signals.push((time, price, signal_type.to_string()));
    }

    /// 计算移动平均线
    fn calculate_ma(&self, period: usize) -> Vec<(DateTime<Utc>, f64)> {
        if self.data.len() < period {
            return Vec::new();
        }

        self.data
            .windows(period)
            .map(|window| {
                let sum: f64 = window.iter().map(|d| d.close).sum();
                (window[period - 1].timestamp, sum / period as f64)
            })
            .collect()
    }

    /// 绘制图表并保存为文件
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let root = BitMapBackend::new(path.as_ref(), (self.style.width, self.style.height))
            .into_drawing_area();
        root.fill(&self.style.background_color)?;

        // 计算价格范围
        let price_min = self
            .data
            .iter()
            .map(|d| d.low)
            .fold(f64::INFINITY, f64::min);
        let price_max = self
            .data
            .iter()
            .map(|d| d.high)
            .fold(f64::NEG_INFINITY, f64::max);
        let price_range = price_max - price_min;
        let price_min = price_min - price_range * 0.1;
        let price_max = price_max + price_range * 0.1;

        // 计算时间范围，增加一点边距
        let time_range = (self.data[self.data.len() - 1].timestamp - self.data[0].timestamp)
            .num_seconds() as f64;
        let time_margin = Duration::seconds((time_range / 20.0) as i64); // 添加5%的时间边距
        let start_time = self.data[0].timestamp - time_margin;
        let end_time = self.data[self.data.len() - 1].timestamp + time_margin;

        // 创建图表
        let mut chart = ChartBuilder::on(&root)
            .caption("Market Data", ("sans-serif", 20).into_font())
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(start_time..end_time, price_min..price_max)?;

        // 配置网格
        let mut mesh = chart.configure_mesh();

        // 设置网格线样式
        mesh.axis_desc_style(("sans-serif", 15))
            .x_label_style(("sans-serif", 12))
            .y_label_style(("sans-serif", 12));

        // 根据数据点数量动态调整网格线数量
        let num_points = self.data.len();
        let x_labels = if num_points <= 10 {
            num_points
        } else if num_points <= 20 {
            num_points / 2
        } else if num_points <= 50 {
            num_points / 5
        } else {
            num_points / 10
        };

        // 设置X轴标签数量
        mesh.x_labels(x_labels);

        // 设置Y轴标签数量（根据价格范围动态调整）
        let price_step = (price_range / 5.0).round(); // 将价格范围分成5份
        mesh.y_labels(5).y_label_formatter(&|y| format!("{:.2}", y));

        // 绘制网格
        mesh.draw()?;

        // 计算每个蜡烛的宽度（使用时间间隔的40%）
        let time_width = Duration::seconds((time_range / (self.data.len() as f64) * 0.4) as i64);

        // 绘制K线图
        for d in &self.data {
            let color = if d.close >= d.open {
                self.style.bull_color
            } else {
                self.style.bear_color
            };

            // 绘制实体
            chart.draw_series(std::iter::once(Rectangle::new(
                [
                    (d.timestamp - time_width, d.open),
                    (d.timestamp + time_width, d.close),
                ],
                color.filled(),
            )))?;

            // 绘制上下影线
            chart.draw_series(std::iter::once(PathElement::new(
                vec![(d.timestamp, d.low), (d.timestamp, d.high)],
                color.stroke_width(2),
            )))?;
        }

        // 绘制均线
        for (i, &period) in self.ma_periods.iter().enumerate() {
            let ma_data = self.calculate_ma(period);
            let color = self.style.ma_colors[i % self.style.ma_colors.len()];
            chart.draw_series(LineSeries::new(ma_data.into_iter(), color.stroke_width(2)))?;
        }

        // 绘制交易信号
        for (time, price, signal_type) in &self.signals {
            let color = if signal_type == "BUY" {
                self.style.signal_colors.0
            } else {
                self.style.signal_colors.1
            };

            // 绘制信号点，买入信号用三角形，卖出信号用倒三角形
            let shape = if signal_type == "BUY" {
                // 绘制向上的三角形
                PathElement::new(
                    vec![
                        (*time, *price - 5.0),                               // 底部中点
                        (time.clone() - Duration::minutes(3), *price + 5.0), // 左上角
                        (time.clone() + Duration::minutes(3), *price + 5.0), // 右上角
                        (*time, *price - 5.0),                               // 回到底部中点
                    ],
                    color.filled(),
                )
            } else {
                // 绘制向下的三角形
                PathElement::new(
                    vec![
                        (*time, *price + 5.0),                               // 顶部中点
                        (time.clone() - Duration::minutes(3), *price - 5.0), // 左下角
                        (time.clone() + Duration::minutes(3), *price - 5.0), // 右下角
                        (*time, *price + 5.0),                               // 回到顶部中点
                    ],
                    color.filled(),
                )
            };

            chart.draw_series(std::iter::once(shape))?;
        }

        root.present()?;
        Ok(())
    }

    /// 获取所有市场数据
    pub fn get_data(&self) -> &[MarketData] {
        &self.data
    }

    /// 获取第一个数据点
    pub fn first_data(&self) -> Option<&MarketData> {
        self.data.first()
    }

    /// 获取最后一个数据点
    pub fn last_data(&self) -> Option<&MarketData> {
        self.data.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use data::DataSource;

    #[test]
    fn test_chart_plotter() {
        // 创建测试数据
        let mut data = Vec::new();
        let start_time = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();

        for i in 0..20 {
            let time = start_time + chrono::Duration::days(i);
            data.push(MarketData {
                symbol: "AAPL".to_string(),
                timestamp: time,
                open: 100.0 + i as f64,
                high: 105.0 + i as f64,
                low: 95.0 + i as f64,
                close: 102.0 + i as f64,
                volume: 1000000.0,
                source: DataSource::Local,
            });
        }

        // 创建图表
        let mut plotter = ChartPlotter::new(data);

        // 添加一些交易信号
        plotter.add_signal(start_time + chrono::Duration::days(5), 105.0, "BUY");
        plotter.add_signal(start_time + chrono::Duration::days(15), 115.0, "SELL");

        // 保存图表
        plotter.save_to_file("test_chart.png").unwrap();
    }
}

// 重新导出 RGBColor 类型
pub type RGBColor = plotters::style::RGBColor;
