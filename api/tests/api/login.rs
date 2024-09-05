use serde_json::json;

use crate::common::spawn_server;

// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn login_returns_200_status() {
    let server = spawn_server().await;
    let url = format!("{}/v1/auth/login", server.addr);

    // Payload (Uses 'Reviewer' test user credentials)
    let body = json!({
        "email": server.test_users[0].email,
        "password": server.test_users[0].password
    });

    // 1. Login Request
    let response = server.post_request(&url, body.to_string()).await;
    assert_eq!(200, response.status().as_u16());
}
// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn login_invalid_user_is_rejected() {
    let server = spawn_server().await;
    let url = format!("{}/v1/auth/login", server.addr);

    // Payload (User should not exist in db)
    let body = json!({
        "email": "test",
        "password": "test"
    });

    // 1. Login Request
    let response = server.post_request(&url, body.to_string()).await;
    assert_eq!(401, response.status().as_u16());
}
// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn sql_injection_login_attempts_are_rejected() {
    let server = spawn_server().await;
    let url = format!("{}/v1/auth/login", server.addr);

    // Test cases with various SQL injection payloads
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
                "email": "test' DROP TABLE 'user'; --",
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

    // 1. Login Requests
    for (invalid_body, error_message) in test_cases {
        let response = server.post_request(&url, invalid_body.to_string()).await;

        (
            assert_eq!(401, response.status().as_u16()),
            "API did not fail with a 401 status when the payload was {}",
            error_message,
        );
    }
}
