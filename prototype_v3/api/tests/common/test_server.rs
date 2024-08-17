use api::web::server;
use api::Config;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

pub struct TestServer {
    pub addr: String,
    #[allow(dead_code)]
    pub db_pool: PgPool,
}

pub async fn spawn_server() -> TestServer {
    dotenvy::dotenv().ok();

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
pub async fn config_database(config: &Config) -> PgPool {
    let mut db_connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .unwrap();

    db_connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.postgres_db).as_str())
        .await
        .unwrap();

    // Run Migrations
    let db_pool = PgPool::connect(&config.connection_string()).await.unwrap();

    sqlx::migrate!("./migrations").run(&db_pool).await.unwrap();

    db_pool
}
