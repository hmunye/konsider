mod common;

use common::spawn_server;

#[tokio::test]
async fn health_check_test() {
    let client = reqwest::Client::new();
    let server = spawn_server().await;
    let url = format!("{}/health-check", server.addr);

    let response = client
        .get(&url)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
    assert_eq!(Some(10), response.content_length());
}
