use std::env;
use std::path::Path;

use api::telemetry::{get_subscriber, init_subscriber};
use api::web::server;
use api::{Config, Environment};

use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    // std::io::sink will not output logs
    let subscriber = get_subscriber("konsider_api".into(), "debug".into(), std::io::stdout);
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

    let db_pool = PgPoolOptions::new()
        // The amount of time the pool will wait to aquire a connection
        .acquire_timeout(std::time::Duration::from_secs(10))
        // The amount of time a connection can stay idle in the pool before it is closed
        .idle_timeout(std::time::Duration::from_secs(60))
        // Won't connect until a
        .connect_lazy(config.connection_string().expose_secret())
        .unwrap();

    let tcp_listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.server_host, config.server_port))
            .await
            .unwrap();

    tracing::info!(
        "->> {:<12} - {}",
        "LISTENING",
        tcp_listener.local_addr().unwrap()
    );

    server::serve(tcp_listener, db_pool).await.unwrap()
}
