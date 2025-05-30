use anyhow::Result;
use common::types::Config;
use core::CoreService;
use log::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    common::utils::init_logging();

    // Initialize utils
    utils::init()?;

    // Create configuration
    let config = Config {
        name: "my-app".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    // Create and start core service
    let service = CoreService::new(config);
    service.start().await?;

    // Example usage
    let result = service.process("Hello, World!").await?;
    info!("Processing result: {}", result);

    // Example of using utils
    let capitalized = utils::string::capitalize("hello");
    info!("Capitalized string: {}", capitalized);

    Ok(())
}
