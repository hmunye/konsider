use std::env;
use std::path::Path;

use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use once_cell::sync::Lazy;
use reqwest::header;
use secrecy::ExposeSecret;
use serde::Serialize;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

use api::server::{get_db_pool, Application};
use api::telemetry::{get_subscriber, init_subscriber};
use api::{Config, Environment, UserRole};

// Ensure it is only initialized once
static TRACING: Lazy<()> = Lazy::new(|| {
    // Using std::io::sink will not output logs
    let subscriber = get_subscriber("test".into(), "info".into(), std::io::sink);
    init_subscriber(subscriber);
});

// ---------------------------------------------------------------------------------------------------------------
pub struct TestServer {
    pub addr: String,
    pub db_pool: PgPool,
    pub test_users: Vec<TestUser>,
    pub api_client: reqwest::Client,
}

impl TestServer {
    pub async fn get_request(&self, url: &String) -> reqwest::Response {
        self.api_client
            .get(url)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_request(&self, url: &String, body: String) -> reqwest::Response {
        self.api_client
            .post(url)
            .header(header::CONTENT_TYPE, "application/json")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_cookie_with_body(
        &self,
        url: &String,
        body: String,
        session_id: &str,
    ) -> reqwest::Response {
        self.api_client
            .post(url)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::COOKIE, session_id)
            .body(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_cookie_without_body(
        &self,
        url: &String,
        session_id: &str,
    ) -> reqwest::Response {
        self.api_client
            .post(url)
            .header(header::COOKIE, session_id)
            .send()
            .await
            .expect("Failed to execute request")
    }
}
// ---------------------------------------------------------------------------------------------------------------
#[derive(Serialize)]
pub struct TestUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
}

impl TestUser {
    pub fn new_reviewer() -> Self {
        let user_role = UserRole::Reviewer;

        Self {
            name: Uuid::new_v4().to_string(),
            email: format!("{}@test.com", Uuid::new_v4().to_string()),
            password: Uuid::new_v4().to_string(),
            role: user_role as UserRole,
        }
    }

    pub fn new_admin() -> Self {
        let user_role = UserRole::Admin;

        Self {
            name: Uuid::new_v4().to_string(),
            email: format!("{}@test.com", Uuid::new_v4().to_string()),
            password: Uuid::new_v4().to_string(),
            role: user_role as UserRole,
        }
    }

    async fn store(&self, db_pool: &PgPool) {
        let salt = SaltString::generate(&mut rand::thread_rng());

        let password_hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),
        )
        .hash_password(self.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

        sqlx::query!(
            r#"
            INSERT INTO users (name, email, password_hash, role)
            VALUES ($1, $2, $3, $4)
            "#,
            Uuid::new_v4().to_string(),
            self.email,
            password_hash,
            self.role.clone() as UserRole
        )
        .execute(db_pool)
        .await
        .expect("Failed to store test user in database");
    }
}
// ---------------------------------------------------------------------------------------------------------------
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

    // Use the same instance of client for each test so there is access to cookies
    let client = reqwest::Client::builder()
        //.cookie_store(true)
        .build()
        .unwrap();

    let test_server = TestServer {
        addr,
        db_pool: get_db_pool(&config),
        test_users: vec![TestUser::new_reviewer(), TestUser::new_admin()],
        api_client: client,
    };

    // test_users[0] is the `Reviewer` test user
    test_server.test_users[0].store(&test_server.db_pool).await;

    // test_users[1] is the `Admin` test user
    test_server.test_users[1].store(&test_server.db_pool).await;

    test_server
}
// ---------------------------------------------------------------------------------------------------------------
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

    // Run migrations
    let db_pool = PgPool::connect(&config.connection_string().expose_secret())
        .await
        .unwrap();

    sqlx::migrate!("./migrations").run(&db_pool).await.unwrap();

    db_pool
}
