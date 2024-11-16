use reqwest::header;
use serde_json::json;

use crate::common::{spawn_server, Result};

#[tokio::test]
async fn login_is_successful() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);

    // Uses 'Reviewer' test user credentials
    let login_body = json!({
        "email": server.test_users[0].email,
        "password": server.test_users[0].password
    });

    let login_response = server
        .post_request(&login_url, Some(login_body.to_string()))
        .await?;
    assert_eq!(204, login_response.status().as_u16());

    let token = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|str| str.split(";").nth(0));
    assert!(token.is_some(), "JWT should be present");

    Ok(())
}

#[tokio::test]
async fn login_with_invalid_credentials_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);

    let login_body = json!({
        "email": "test@sdasdas.com",
        "password": "testsadasdasdsas"
    });

    let login_response = server
        .post_request(&login_url, Some(login_body.to_string()))
        .await?;
    assert_eq!(401, login_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn login_with_missing_fields_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);

    let test_cases = vec![
        (
            json!({
                "password": "testing12323445",
            }),
            "missing email field",
        ),
        (
            json!({
                "email": "testing@123.com",
            }),
            "missing password field",
        ),
        (json!({}), "missing both email and password fields"),
    ];

    for (invalid_body, error_message) in test_cases {
        let login_response = server
            .post_request(&login_url, Some(invalid_body.to_string()))
            .await?;
        (
            assert_eq!(400, login_response.status().as_u16()),
            "API did not fail with a 400 status when the payload was {}",
            error_message,
        );
    }

    Ok(())
}

#[tokio::test]
async fn sql_injection_login_attempts_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);

    let test_cases = vec![
        (
            json!({
                "email": "test' OR '1'='1",
                "password": "testing123",
            }),
            "SQL injection attempt in email",
        ),
        (
            json!({
                "email": "test@gmail.com",
                "password": "password' OR '1'='1",
            }),
            "SQL injection attempt in password",
        ),
        (
            json!({
                "email": "test' DROP TABLE users; --",
                "password": "password",
            }),
            "SQL injection attempt to drop table",
        ),
        (
            json!({
                "email": "test' OR '1'='1' --",
                "password": "password",
            }),
            "SQL injection attempt with comment",
        ),
        (
            json!({
                "email": "test@example.com",
                "password": "password' OR '1'='1' --",
            }),
            "SQL injection attempt with comment in password",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let login_response = server
            .post_request(&login_url, Some(invalid_body.to_string()))
            .await?;
        (
            assert_eq!(401, login_response.status().as_u16()),
            "API did not fail with a 401 status when the payload was {}",
            error_message,
        );
    }

    Ok(())
}
