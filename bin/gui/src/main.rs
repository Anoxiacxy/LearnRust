use anyhow::Result;
use app_core::MarketMonitor;
use data::models::DataProvider;
use data::providers::CryptoDataProvider;
use eframe::egui;
use std::boxed::Box;
use std::time::Duration as StdDuration;

struct MyApp {
    monitor: Option<MarketMonitor>,
    symbol: String,
    interval: u64,
}

impl MyApp {
    fn new() -> Self {
        Self {
            monitor: None,
            symbol: "BTC".to_string(),
            interval: 300,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("量化交易平台 GUI");
            ui.horizontal(|ui| {
                ui.label("Symbol:");
                ui.text_edit_singleline(&mut self.symbol);
            });
            ui.horizontal(|ui| {
                ui.label("Interval (sec):");
                ui.add(egui::Slider::new(&mut self.interval, 1..=600).text("sec"));
            });
            if ui.button("Start Monitor").clicked() {
                let provider: Box<dyn DataProvider> = Box::new(CryptoDataProvider::new());
                let symbol_list = vec![self.symbol.clone()];
                let monitor = MarketMonitor::new(
                    provider,
                    symbol_list,
                    StdDuration::from_secs(self.interval),
                );
                // 这里仅示例，实际中应异步启动 monitor.run()，例如用 tokio::spawn
                self.monitor = Some(monitor);
            }
        });
    }
}

fn main() -> Result<()> {
    env_logger::init();
    let options = eframe::NativeOptions {
        ..eframe::NativeOptions::default()
    };
    eframe::run_native("Quant GUI", options, Box::new(|_cc| Box::new(MyApp::new()))).unwrap();
    Ok(())
}
