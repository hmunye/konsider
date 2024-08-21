use api::Config;

use crate::common::spawn_server;

use secrecy::ExposeSecret;
use sqlx::PgPool;

#[tokio::test]
async fn health_check_test() {
    let server = spawn_server().await;
    let url = format!("{}/health-check", server.addr);

    let config = Config::default();

    // Make sure we can establish connection to db
    let _ = PgPool::connect(&config.connection_string().expose_secret())
        .await
        .unwrap();

    // Request
    let response = server.get_request(&url).await;

    assert_eq!(200, response.status().as_u16());
    assert_eq!(Some(10), response.content_length());
}
