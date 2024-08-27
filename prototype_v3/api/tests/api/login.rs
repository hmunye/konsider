use serde_json::json;

use crate::common::spawn_server;

// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn login_returns_200_status() {
    let server = spawn_server().await;
    let url = format!("{}/auth/login", server.addr);

    // Payload (Uses test user credentials)
    let body = json!({
        "email": server.test_user.email,
        "password": server.test_user.password
    });

    // Request
    let response = server.post_request(&url, body.to_string()).await;

    assert_eq!(200, response.status().as_u16());
}
// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn login_returns_401_status() {
    let server = spawn_server().await;
    let url = format!("{}/auth/login", server.addr);

    // Payload (User should not exist in db)
    let body = json!({
        "email": "test",
        "password": "test"
    });

    // Request
    let response = server.post_request(&url, body.to_string()).await;

    assert_eq!(401, response.status().as_u16());
}
