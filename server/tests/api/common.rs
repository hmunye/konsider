use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use axum::http::header;
use secrecy::SecretString;
use serde::Serialize;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

use k6r::api::UserRole;
use k6r::config::{get_config, DatabaseConfig};
use k6r::log::{get_subscriber, init_subscriber};
use k6r::server::{get_db_pool, Server};

// Type alias for Result
pub type Result<T> = std::result::Result<T, Error>;

// Using `Box<dyn std::error::Error>` for flexibility in error handling
pub type Error = Box<dyn std::error::Error + Send + Sync>;

// Ensure the `tracing` stack is only initialized once
static TRACING: std::sync::LazyLock<()> = std::sync::LazyLock::new(|| {
    // Using std::io::sink will not output logs, std::io::stdout outputs to stdout
    // Depends on the presence of the environment variable `TEST_LOG`
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber("test".into(), "info".into(), std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber("test".into(), "info".into(), std::io::sink);
        init_subscriber(subscriber);
    };
});

#[derive(Debug)]
pub struct TestServer {
    pub addr: String,
    pub db_pool: PgPool,
    pub test_users: Vec<TestUser>,
    pub client: reqwest::Client,
}

// Provides methods for sending various types of HTTP requests: (GET, POST)
// to a specified URL with optional request body
impl TestServer {
    pub async fn get_request(&self, url: &String) -> Result<reqwest::Response> {
        Ok(self
            .client
            .get(url)
            .send()
            .await
            .map_err(|err| format!("failed to execute request. cause: {err}"))?)
    }

    pub async fn post_request(
        &self,
        url: &String,
        body: Option<String>,
    ) -> Result<reqwest::Response> {
        match body {
            Some(body) => Ok(self
                .client
                .post(url)
                .header(header::CONTENT_TYPE, "application/json")
                .body(body)
                .send()
                .await
                .map_err(|err| format!("failed to execute request. cause: {err}"))?),
            None => Ok(self
                .client
                .post(url)
                .send()
                .await
                .map_err(|err| format!("failed to execute request. cause: {err}"))?),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TestUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
}

impl TestUser {
    pub fn new_reviewer() -> Self {
        let user_role = UserRole::REVIEWER;

        Self {
            id: Uuid::new_v4(),
            name: Uuid::new_v4().to_string(),
            email: format!("{}brockport.edu", Uuid::new_v4().to_string()),
            password: Uuid::new_v4().to_string(),
            role: user_role as UserRole,
        }
    }

    pub fn new_admin() -> Self {
        let user_role = UserRole::ADMIN;

        Self {
            id: Uuid::new_v4(),
            name: Uuid::new_v4().to_string(),
            email: format!("{}brockport.edu", Uuid::new_v4().to_string()),
            password: Uuid::new_v4().to_string(),
            role: user_role as UserRole,
        }
    }

    async fn store(&self, db_pool: &PgPool) -> Result<()> {
        let salt = SaltString::generate(&mut rand::thread_rng());

        let password_hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),
        )
        .hash_password(self.password.as_bytes(), &salt)?
        .to_string();

        sqlx::query!(
            r#"
            INSERT INTO user_account (id, name, email, password_hash, role)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            self.id,
            self.name,
            self.email,
            password_hash,
            self.role.clone() as UserRole
        )
        .execute(db_pool)
        .await
        .map_err(|err| format!("failed to store test user in database. cause: {err}"))?;

        Ok(())
    }
}

pub async fn spawn_server() -> Result<TestServer> {
    std::sync::LazyLock::force(&TRACING);

    let config = {
        let mut config = get_config().expect("failed to read config");

        // Use a different database for each test case
        config.database.database = Uuid::new_v4().to_string();

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
        test_users: vec![
            TestUser::new_reviewer(),
            TestUser::new_admin(),
            TestUser::new_reviewer(),
            TestUser::new_admin(),
        ],
        client,
    };

    // test_users[0] is a `Reviewer` test user
    test_server.test_users[0]
        .store(&test_server.db_pool)
        .await?;

    // test_users[1] is an `Admin` test user
    test_server.test_users[1]
        .store(&test_server.db_pool)
        .await?;

    // test_users[2] is a `Reviewer` test user
    test_server.test_users[2]
        .store(&test_server.db_pool)
        .await?;

    // test_users[3] is an `Admin` test user
    test_server.test_users[3]
        .store(&test_server.db_pool)
        .await?;

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

    // Step 1: Connect as the superuser (postgres)
    let mut connection = PgConnection::connect_with(&default_config.connect_options()).await?;

    // Step 2: Create the new database using the superuser
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database).as_str())
        .await?;

    // Step 3: Change the owner of the newly created database to k6r user
    connection
        .execute(
            format!(
                r#"ALTER DATABASE "{}" OWNER TO "{}";"#,
                config.database, config.user
            )
            .as_str(),
        )
        .await?;

    // Step 4: Create connection pool using k6r user
    let connection_pool = PgPool::connect_with(config.connect_options()).await?;

    // Step 5: Run migrations on the new database
    sqlx::migrate!("./migrations").run(&connection_pool).await?;

    Ok(connection_pool)
}
