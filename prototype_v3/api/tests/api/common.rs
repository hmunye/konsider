use std::env;
use std::path::Path;

use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

use api::server::{get_db_pool, Application};
use api::telemetry::{get_subscriber, init_subscriber};
use api::{Config, Environment};

// Ensure it is only initialized once
static TRACING: Lazy<()> = Lazy::new(|| {
    // Using std::io::sink will not output logs
    let subscriber = get_subscriber("test".into(), "info".into(), std::io::sink);
    init_subscriber(subscriber);
});

pub struct TestServer {
    pub addr: String,
    pub db_pool: PgPool,
}

impl TestServer {
    pub async fn get_request(&self, url: &String) -> reqwest::Response {
        reqwest::Client::new()
            .get(url)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_request(&self, url: &String, body: String) -> reqwest::Response {
        reqwest::Client::new()
            .post(url)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request")
    }
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

    let config = {
        let mut config = Config::default();

        // Use a different database for each test case
        config.postgres_db = Uuid::new_v4().to_string();

        // Using port '0' will trigger the OS to scan for an available port
        // This allows the server to continue running on port 8000 while each test is executed using
        // a different port. Avoids port conflicts
        config.server_port = 0;

        config
    };

    config_database(&config).await;

    let application = Application::build(config.clone())
        .await
        .expect("Failed to build application");

    let addr = format!("http://{}:{}", application.host(), application.port());

    // Spawns a new asynchronous task
    // Used to start a new task that starts a new instance of the server
    tokio::spawn(async move { application.run_server().await });

    TestServer {
        addr,
        db_pool: get_db_pool(&config),
    }
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
