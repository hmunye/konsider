mod common;

use common::spawn_server;
use serde_json::json;

#[tokio::test]
async fn login_returns_200_status() {
    let client = reqwest::Client::new();
    let server = spawn_server().await;
    let url = format!("{}/auth/login", server.addr);

    let body = json!({
        "email": "test",
        "password": "test"
    });

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
    assert_eq!(Some(16), response.content_length());
}

#[tokio::test]
async fn login_returns_500_status() {
    let client = reqwest::Client::new();
    let server = spawn_server().await;
    let url = format!("{}/auth/login", server.addr);

    let body = json!({
        "email": "",
        "password": ""
    });

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(500, response.status().as_u16());
}
