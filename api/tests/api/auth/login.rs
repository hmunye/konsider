// TODO: Test to verify session token expiration and validate session token

use reqwest::header;
use serde_json::json;

use crate::common::spawn_server;

// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn login_is_successful_and_returns_session_token() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);

    // Uses 'Reviewer' test user credentials
    let body = json!({
        "email": server.test_users[0].email,
        "password": server.test_users[0].password
    });

    let login_response = server
        .post_request(&login_url, Some(body.to_string()), None, None)
        .await;
    assert_eq!(200, login_response.status().as_u16());

    let session_id = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|str| str.split(";").nth(0));
    assert!(session_id.is_some(), "Session ID should be present");
}
// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn login_with_invalid_credentials_rejected() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);

    let body = json!({
        "email": "test@sdasdas.com",
        "password": "testsadasdasdsas"
    });

    let response = server
        .post_request(&login_url, Some(body.to_string()), None, None)
        .await;
    assert_eq!(401, response.status().as_u16());
}
// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn login_with_missing_credentials_rejected() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);

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
        let response = server
            .post_request(&login_url, Some(invalid_body.to_string()), None, None)
            .await;

        (
            assert_eq!(400, response.status().as_u16()),
            "API did not fail with a 400 status when the payload was {}",
            error_message,
        );
    }
}
// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn sql_injection_login_attempts_rejected() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);

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
        let response = server
            .post_request(&login_url, Some(invalid_body.to_string()), None, None)
            .await;

        (
            assert_eq!(401, response.status().as_u16()),
            "API did not fail with a 401 status when the payload was {}",
            error_message,
        );
    }
}
