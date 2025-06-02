use crate::Result;
use log::info;
use serde_json::Value;

pub mod file {
    use super::*;
    use std::path::Path;

    pub fn read_json_file<P: AsRef<Path>>(path: P) -> Result<Value> {
        let content = std::fs::read_to_string(path)?;
        let value = serde_json::from_str(&content)?;
        Ok(value)
    }
}

pub mod string {
    pub fn capitalize(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().chain(chars).collect(),
        }
    }
}

pub fn init() -> Result<()> {
    info!("Utils module initialized");
    Ok(())
}

pub fn init_logging() {
    env_logger::init();
    info!("Logging initialized");
}
