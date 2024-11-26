use reqwest::header;
use serde_json::json;
use uuid::Uuid;

use crate::common::{spawn_server, Result};

#[tokio::test]
async fn change_user_password_successful() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);
    let change_password_url = format!("{}/api/v1/users/password", server.addr);

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

    let request_body = json!({
        "current_password": server.test_users[1].password,
        "new_password": "password123"
    });

    let change_password_response = server
        .post_request(
            &change_password_url,
            Some(request_body.to_string()),
            Some(token.unwrap()),
        )
        .await?;
    assert_eq!(204, change_password_response.status().as_u16());

    // Attempt to login with old password
    let dup_login_response = server
        .post_request(&login_url, Some(login_body.to_string()), None)
        .await?;
    assert_eq!(401, dup_login_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn change_password_using_invalid_token_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let change_password_url = format!("{}/api/v1/users/password", server.addr);

    let request_body = json!({
        "current_password": server.test_users[1].password,
        "new_password": "password123"
    });

    let change_password_response = server
        .post_request(
            &change_password_url,
            Some(request_body.to_string()),
            Some(&Uuid::new_v4().to_string()),
        )
        .await?;
    assert_eq!(401, change_password_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn change_password_using_missing_token_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let change_password_url = format!("{}/api/v1/users/password", server.addr);

    let request_body = json!({
        "current_password": server.test_users[1].password,
        "new_password": "password123"
    });

    let change_password_response = server
        .post_request(&change_password_url, Some(request_body.to_string()), None)
        .await?;
    assert_eq!(401, change_password_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn change_password_using_invalid_password_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);
    let change_password_url = format!("{}/api/v1/users/password", server.addr);

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

    let test_cases = vec![
        (
            json!({
                "current_password": server.test_users[1].password,
                "new_password": "pas"
            }),
            "new password length too short",
        ),
        (
            json!({
                "current_password": server.test_users[1].password,
                "new_password": "  "
            }),
            "empty new password",
        ),
        (
            json!({
                "current_password": server.test_users[1].password,
                "new_password": "a".repeat(129)
            }),
            "new password length too long",
        ),
        (
            json!({
                "current_password": server.test_users[1].password,
                "new_password": "p/ass(w)o<r\\"
            }),
            "password contains forbidden characters",
        ),
    ];
    for (invalid_body, error_message) in test_cases {
        let change_password_response = server
            .post_request(
                &change_password_url,
                Some(invalid_body.to_string()),
                Some(token.unwrap()),
            )
            .await?;
        (assert_eq!(
            400,
            change_password_response.status().as_u16(),
            "API did not fail with a 400 status when the payload was {}",
            error_message,
        ),);
    }

    Ok(())
}

#[tokio::test]
async fn change_password_using_missing_fields_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);
    let change_password_url = format!("{}/api/v1/users/password", server.addr);

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

    let test_cases = vec![
        (
            json!({
                "new_password": "password123"
            }),
            "missing current password",
        ),
        (
            json!({
                "current_password": server.test_users[1].password,
            }),
            "missing new password",
        ),
        (json!({}), "missing both fields"),
    ];
    for (invalid_body, error_message) in test_cases {
        let change_password_response = server
            .post_request(
                &change_password_url,
                Some(invalid_body.to_string()),
                Some(token.unwrap()),
            )
            .await?;
        (assert_eq!(
            400,
            change_password_response.status().as_u16(),
            "API did not fail with a 400 status when the payload was {}",
            error_message,
        ),);
    }

    Ok(())
}
