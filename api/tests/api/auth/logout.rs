// TODO: Test with session token that is expired

use reqwest::header;
use serde_json::json;
use uuid::Uuid;

use crate::common::spawn_server;

// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn logout_is_successful_and_clears_session() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);
    let logout_url = format!("{}/v1/auth/logout", server.addr);
    let users_url = format!("{}/v1/users", server.addr);

    // Uses 'Admin' test user credentials
    let body = json!({
        "email": server.test_users[1].email,
        "password": server.test_users[1].password
    });

    let login_response = server
        .post_request(&login_url, Some(body.to_string()), None, None)
        .await;
    assert_eq!(204, login_response.status().as_u16());

    let session_id = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|str| str.split(";").nth(0));
    assert!(session_id.is_some(), "Session ID should be present");

    let logout_response = server
        .post_request(&logout_url, None, Some(&session_id.unwrap()), None)
        .await;
    assert_eq!(204, logout_response.status().as_u16());

    let email: String = String::from("john@gmail.com");

    let body = json!({
        "name": "John",
        "email": &email,
        "password": "password1234",
        "role": "Reviewer"
    });

    // Using cleared session id on an endpoint that requires a valid session
    let create_user_response = server
        .post_request(
            &users_url,
            Some(body.to_string()),
            Some(&session_id.unwrap()),
            None,
        )
        .await;
    assert_eq!(401, create_user_response.status().as_u16());
}
// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn logout_with_invalid_session_token_rejected() {
    let server = spawn_server().await;
    let logout_url = format!("{}/v1/auth/logout", server.addr);

    let session_id = Uuid::new_v4().to_string();

    let logout_response = server
        .post_request(&logout_url, None, Some(&session_id), None)
        .await;
    assert_eq!(401, logout_response.status().as_u16());
}
// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn logout_with_missing_session_token_rejected() {
    let server = spawn_server().await;
    let logout_url = format!("{}/v1/auth/logout", server.addr);

    let logout_response = server.post_request(&logout_url, None, None, None).await;
    assert_eq!(401, logout_response.status().as_u16());
}
