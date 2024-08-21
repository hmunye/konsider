use std::env;
use std::path::Path;

use api::server::Application;
use api::telemetry::{get_subscriber, init_subscriber};
use api::{Config, Environment};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // std::io::sink will not output logs
    let subscriber = get_subscriber("konsider_api".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Detect the running environment. Defaults to local if not provided
    let environment: Environment = env::var("ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .unwrap();

    let env_file = match environment.as_str() {
        "production" => ".env.production",
        _ => ".env.local",
    };

    // Load the specified .env file
    let _ = dotenvy::from_path(Path::new(env_file));

    let config = Config::default();

    let application = Application::build(config.clone())
        .await
        .expect("Failed to build application");

    application.run_server().await?;

    Ok(())
}
