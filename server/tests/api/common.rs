use k6r::config::{get_config, DatabaseConfig};
use k6r::server::{get_db_pool, Server};
use secrecy::SecretString;
use sqlx::{Connection, Executor, PgConnection, PgPool};

// Type alias for Result
pub type Result<T> = std::result::Result<T, Error>;

// Using `Box<dyn std::error::Error>` for flexibility in error handling
pub type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
#[allow(unused)]
pub struct TestServer {
    pub addr: String,
    pub db_pool: PgPool,
    pub client: reqwest::Client,
}

// Provides method for sending GET HTTP requests to a specified URL
impl TestServer {
    pub async fn get_request(&self, url: &String) -> Result<reqwest::Response> {
        Ok(self
            .client
            .get(url)
            .send()
            .await
            .map_err(|err| format!("failed to execute request: cause {err}"))?)
    }
}

pub async fn spawn_server() -> Result<TestServer> {
    let config = {
        let mut config = get_config().expect("failed to read config");

        // Use a different database for each test case
        config.database.database = uuid::Uuid::new_v4().to_string();

        // Using port '0' will trigger the OS to scan for an available port
        // This allows the server to continue running on port 8000 while each test is executed using
        // a different port. Avoids port conflicts
        config.server.port = 0;

        config
    };

    config_database(&config.database).await?;

    let server = Server::build(config.clone()).await?;

    let port = server.port();

    // Spawn a new asynchronous task using `tokio::spawn`.
    // Creates a non-blocking task that runs the server instance in the background.
    // The server's `run` method is awaited within this task, allowing it to
    // handle incoming requests while the main thread can continue executing
    tokio::spawn(server.run());

    let client = reqwest::Client::builder().build()?;

    let test_server = TestServer {
        addr: format!("http://127.0.0.1:{}", port),
        db_pool: get_db_pool(&config.database)?,
        client,
    };

    Ok(test_server)
}

// Create a new database for each test with a unique name for better test isolation
async fn config_database(config: &DatabaseConfig) -> Result<PgPool> {
    let default_config = DatabaseConfig {
        user: "postgres".into(),
        password: SecretString::new("password".into()),
        database: "postgres".into(),
        ..config.clone()
    };

    let mut connection = PgConnection::connect_with(&default_config.connect_options()).await?;

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database).as_str())
        .await?;

    let connection_pool = PgPool::connect_with(config.connect_options()).await?;

    sqlx::migrate!("./migrations").run(&connection_pool).await?;

    Ok(connection_pool)
}
