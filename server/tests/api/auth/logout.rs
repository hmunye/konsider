use reqwest::header;
use serde_json::json;
use uuid::Uuid;

use crate::common::{spawn_server, Result};

#[tokio::test]
async fn logout_is_successful() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);
    let logout_url = format!("{}/api/v1/auth/logout", server.addr);

    // Uses 'Admin' test user credentials
    let login_body = json!({
        "email": server.test_users[1].email,
        "password": server.test_users[1].password
    });

    let login_response = server
        .post_request(&login_url, Some(login_body.to_string()), None)
        .await?;
    assert_eq!(204, login_response.status().as_u16());

    let token = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok());
    assert!(token.is_some(), "JWT should be present");

    let logout_response = server
        .post_request(&logout_url, None, Some(token.unwrap()))
        .await?;
    assert_eq!(204, logout_response.status().as_u16());

    // Using revoked JWT on an endpoint that requires a valid token
    let dup_logout_response = server
        .post_request(&logout_url, None, Some(token.unwrap()))
        .await?;
    assert_eq!(401, dup_logout_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn logout_with_invalid_token_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let logout_url = format!("{}/api/v1/auth/logout", server.addr);

    let logout_response = server
        .post_request(&logout_url, None, Some(&Uuid::new_v4().to_string()))
        .await?;
    assert_eq!(401, logout_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn logout_with_missing_token_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let logout_url = format!("{}/api/v1/auth/logout", server.addr);

    let logout_response = server.post_request(&logout_url, None, None).await?;
    assert_eq!(401, logout_response.status().as_u16());

    Ok(())
}
