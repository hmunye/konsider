mod common;

use api::Config;
use common::spawn_server;
use sqlx::PgPool;

#[tokio::test]
async fn health_check_test() {
    let client = reqwest::Client::new();
    let server = spawn_server().await;
    let url = format!("{}/health-check", server.addr);

    let config = Config::default();

    let connection_string = config.connection_string();

    // Make sure we can connect to the db
    let _ = PgPool::connect(&connection_string).await.unwrap();

    let response = client
        .get(&url)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
    assert_eq!(Some(10), response.content_length());
}
