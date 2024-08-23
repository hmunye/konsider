use std::env;
use std::path::Path;

use api::server::Application;
use api::telemetry::{get_subscriber, init_subscriber};
use api::{Config, Environment};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // This creates an hourly rotating file appender that writes to /logs/konsider_api.YYYY-MM-DD-HH
    let file_appender = tracing_appender::rolling::hourly("./logs", "konsider_api");

    // This spawns a dedicated worker thread which is responsible for writing log lines to the provided writer.
    // _guard ensures buffered logs are flushed to their output in the case of abrupt terminations
    let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);

    // Using std::io::sink will not output logs
    let subscriber = get_subscriber("konsider_api".into(), "debug".into(), file_writer);
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

    tracing::info!(
        "->> {} ON - {}",
        "LISTENING",
        format!("{}:{}", &application.host(), &application.port())
    );

    application.run_server().await?;

    Ok(())
}
