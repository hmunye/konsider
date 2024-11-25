use axum::http::header;
use serde_json::json;

use crate::common::{spawn_server, Result};

#[tokio::test]
async fn check_token_is_successful() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);
    let check_token_url = format!("{}/api/v1/auth/check", server.addr);

    // Uses 'Reviewer' test user credentials
    let login_body = json!({
        "email": server.test_users[0].email,
        "password": server.test_users[0].password
    });

    let login_response = server
        .post_request(&login_url, Some(login_body.to_string()), None)
        .await?;
    assert_eq!(200, login_response.status().as_u16());

    let token = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|str| str.split(";").nth(0));
    assert!(token.is_some(), "JWT should be present");

    let check_token_response = server
        .get_request(&check_token_url, Some(&token.unwrap()))
        .await?;
    assert_eq!(200, check_token_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn check_token_with_invalid_token_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let check_token_url = format!("{}/api/v1/auth/check", server.addr);

    let check_token_response = server
        .get_request(&check_token_url, Some(&uuid::Uuid::new_v4().to_string()))
        .await?;
    assert_eq!(401, check_token_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn check_token_with_missing_token_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let check_token_url = format!("{}/api/v1/auth/check", server.addr);

    let check_token_response = server.get_request(&check_token_url, None).await?;
    assert_eq!(401, check_token_response.status().as_u16());

    Ok(())
}
