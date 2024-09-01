use reqwest::header;
use serde_json::json;

use crate::common::spawn_server;

// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn logout_clears_session_state() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);
    let logout_url = format!("{}/v1/auth/logout", server.addr);

    // Payload (Uses 'Reviewer' test user credentials)
    let body = json!({
        "email": server.test_users[0].email,
        "password": server.test_users[0].password
    });

    // 1. Login Request
    let login_response = server.post_request(&login_url, body.to_string()).await;
    assert_eq!(200, login_response.status().as_u16());

    // TODO: Find out how to correctly preserve cookies without manual extraction
    let session_id = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|str| str.split(";").nth(0));

    // 2. Logout Request
    let logout_response = server.post_logout(&logout_url, &session_id.unwrap()).await;
    assert_eq!(200, logout_response.status().as_u16());
}
