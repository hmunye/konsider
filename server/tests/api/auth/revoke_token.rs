use axum::http::header;
use serde_json::json;

use crate::common::{spawn_server, Result};

#[tokio::test]
async fn revoke_token_is_successful() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);
    let check_token_url = format!("{}/api/v1/auth/check", server.addr);

    // Uses 'Reviewer' test user id
    let test_user_id = server.test_users[0].id;
    let revoke_token_url = format!("{}/api/v1/auth/revoke/{}", server.addr, test_user_id);

    // Uses 'Admin' test user credentials
    let login_body = json!({
        "email": server.test_users[1].email,
        "password": server.test_users[1].password
    });

    let login_response = server
        .post_request(&login_url, Some(login_body.to_string()), None)
        .await?;
    assert_eq!(204, login_response.status().as_u16());

    let admin_token = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|str| str.split(";").nth(0));
    assert!(admin_token.is_some(), "JWT should be present");

    // Uses 'Reviewer' test user credentials
    let login_body = json!({
        "email": server.test_users[0].email,
        "password": server.test_users[0].password
    });

    let login_response = server
        .post_request(&login_url, Some(login_body.to_string()), None)
        .await?;
    assert_eq!(204, login_response.status().as_u16());

    let reviewer_token = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|str| str.split(";").nth(0));
    assert!(reviewer_token.is_some(), "JWT should be present");

    let check_token_response = server
        .get_request(&check_token_url, Some(&reviewer_token.unwrap()))
        .await?;
    assert_eq!(200, check_token_response.status().as_u16());

    let revoke_token_response = server
        .delete_request(&revoke_token_url, Some(&admin_token.unwrap()))
        .await?;
    assert_eq!(204, revoke_token_response.status().as_u16());

    let check_token_response = server
        .get_request(&check_token_url, Some(&reviewer_token.unwrap()))
        .await?;
    assert_eq!(401, check_token_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn revoke_token_with_invalid_token_rejected() -> Result<()> {
    let server = spawn_server().await?;

    // Uses 'Reviewer' test user id
    let test_user_id = server.test_users[0].id;
    let revoke_token_url = format!("{}/api/v1/auth/revoke/{}", server.addr, test_user_id);

    let revoke_token_response = server
        .delete_request(&revoke_token_url, Some(&uuid::Uuid::new_v4().to_string()))
        .await?;
    assert_eq!(401, revoke_token_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn revoke_token_with_missing_token_rejected() -> Result<()> {
    let server = spawn_server().await?;

    // Uses 'Reviewer' test user id
    let test_user_id = server.test_users[0].id;
    let revoke_token_url = format!("{}/api/v1/auth/revoke/{}", server.addr, test_user_id);

    let revoke_token_response = server.delete_request(&revoke_token_url, None).await?;
    assert_eq!(401, revoke_token_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn revoke_token_with_invalid_role_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);

    // Uses 'Reviewer' test user id
    let test_user_id = server.test_users[0].id;
    let revoke_token_url = format!("{}/api/v1/auth/revoke/{}", server.addr, test_user_id);

    // Uses 'Reviewer' test user credentials
    let login_body = json!({
        "email": server.test_users[0].email,
        "password": server.test_users[0].password
    });

    let login_response = server
        .post_request(&login_url, Some(login_body.to_string()), None)
        .await?;
    assert_eq!(204, login_response.status().as_u16());

    let token = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|str| str.split(";").nth(0));
    assert!(token.is_some(), "JWT should be present");

    let revoke_token_response = server
        .delete_request(&revoke_token_url, Some(&token.unwrap()))
        .await?;
    assert_eq!(403, revoke_token_response.status().as_u16());

    Ok(())
}
