use reqwest::header;
use serde_json::json;
use uuid::Uuid;

use crate::common::{spawn_server, Result};

#[tokio::test]
async fn delete_user_successful() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);

    // Uses 'Reviewer' test user id
    let test_user_id = server.test_users[0].id;
    let users_url = format!("{}/api/v1/users/{}", server.addr, test_user_id);

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

    let delete_user_response = server
        .delete_request(&users_url, Some(&token.unwrap()))
        .await?;
    assert_eq!(204, delete_user_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn delete_user_using_invalid_role_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);

    // Uses 'Reviewer' test user delete id
    let test_user_id = server.test_users[0].id;
    let users_url = format!("{}/api/v1/users/{}", server.addr, test_user_id);

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
        .and_then(|value| value.to_str().ok());
    assert!(token.is_some(), "JWT should be present");

    let delete_user_response = server
        .delete_request(&users_url, Some(&token.unwrap()))
        .await?;
    assert_eq!(403, delete_user_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn delete_user_with_invalid_id_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);

    let test_user_id = Uuid::new_v4();
    let users_url = format!("{}/api/v1/users/{}", server.addr, test_user_id);

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

    let delete_user_response = server
        .delete_request(&users_url, Some(&token.unwrap()))
        .await?;
    // Returns a 404 status code to indicate the user does not exist
    assert_eq!(404, delete_user_response.status().as_u16());

    Ok(())
}
