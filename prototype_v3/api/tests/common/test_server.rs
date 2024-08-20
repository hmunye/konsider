use std::env;
use std::path::Path;

use api::telemetry::{get_subscriber, init_subscriber};
use api::web::server;
use api::{Config, Environment};

use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

// Ensure it is only initialized once
static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber = get_subscriber("test".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
});

pub struct TestServer {
    pub addr: String,
    #[allow(dead_code)]
    pub db_pool: PgPool,
}

pub async fn spawn_server() -> TestServer {
    Lazy::force(&TRACING);

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

    let mut config = Config::default();

    // Create unique database name
    config.postgres_db = Uuid::new_v4().to_string();

    // Using port '0' will trigger the OS to scan for an available port
    // This allows the server to continue running on port 8000 while each test is executed using
    // a different port. Avoids port conflicts
    let addr = format!("{}:0", config.server_host);

    let db_pool = config_database(&config).await;

    let tcp_listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    let port = tcp_listener.local_addr().unwrap().port();

    let server = server::serve(tcp_listener, db_pool.clone());

    // Spawns a new asynchronous task
    // Used to start a new task that starts a new instance of the server
    tokio::spawn(async move { server.await.unwrap() });

    let addr = format!("http://{}:{}", config.server_host, port);

    TestServer { addr, db_pool }
}

// Create a new database for each test with a unique name for better test isolation
async fn config_database(config: &Config) -> PgPool {
    let mut db_connection =
        PgConnection::connect(&config.connection_string_without_db().expose_secret())
            .await
            .unwrap();

    db_connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.postgres_db).as_str())
        .await
        .unwrap();

    // Run Migrations
    let db_pool = PgPool::connect(&config.connection_string().expose_secret())
        .await
        .unwrap();

    sqlx::migrate!("./migrations").run(&db_pool).await.unwrap();

    db_pool
}
