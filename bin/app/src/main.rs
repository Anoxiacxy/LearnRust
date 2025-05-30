use anyhow::Result;
use common::{types::Config, Container};
use core::{CoreService, UserService, UserServiceImpl};
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

    // Create and configure DI container
    let container = Container::new();

    // Register services
    container.register(CoreService::new(config.clone()))?;
    container.register(UserServiceImpl::new())?;

    // Resolve and use services
    let core_service: CoreService = container.resolve()?;
    let user_service: UserServiceImpl = container.resolve()?;

    // Start core service
    core_service.start().await?;

    // Example usage of core service
    let result = core_service.process("Hello, World!").await?;
    info!("Processing result: {}", result);

    // Example usage of user service
    let user = user_service
        .create_user("John Doe".to_string(), "john@example.com".to_string())
        .await?;
    info!("Created user: {:?}", user);

    let retrieved_user = user_service.get_user(user.id).await?;
    info!("Retrieved user: {:?}", retrieved_user);

    // Example of using utils
    let capitalized = utils::string::capitalize("hello");
    info!("Capitalized string: {}", capitalized);

    Ok(())
}
