use reqwest::header;
use serde_json::json;

use crate::common::spawn_server;

// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn create_user_returns_200_status() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);
    let url = format!("{}/v1/admin/create-user", server.addr);

    // Payload (Uses 'Admin' test user credentials)
    let body = json!({
        "email": server.test_users[1].email,
        "password": server.test_users[1].password
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

    let email: String = String::from("john@gmail.com");

    // Payload
    let body = json!({
        "name": "John",
        "email": &email,
        "password": "password1234",
        "role": "Reviewer"
    });

    // 2. Create User Request
    let create_user_response = server
        .post_create_user(&url, body.to_string(), &session_id.unwrap())
        .await;

    assert_eq!(200, create_user_response.status().as_u16());

    // 3. Check for user in database
    let row = sqlx::query!(
        r#"
        SELECT id
        FROM users
        WHERE email=$1 
        "#,
        email
    )
    .fetch_one(&server.db_pool)
    .await;

    assert!(row.is_ok());
}
// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn create_user_invalid_role_is_rejected() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);
    let url = format!("{}/v1/admin/create-user", server.addr);

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

    let email: String = String::from("john@gmail.com");

    // Payload
    let body = json!({
        "name": "John",
        "email": &email,
        "password": "password123",
        "role": "Reviewer"
    });

    // 2. Create User Request (Should fail because the test user has role `Reviewer`)
    let create_user_response = server
        .post_create_user(&url, body.to_string(), &session_id.unwrap())
        .await;

    // 3. Check for user in database
    let row = sqlx::query!(
        r#"
        SELECT id
        FROM users
        WHERE email=$1 
        "#,
        email
    )
    .fetch_one(&server.db_pool)
    .await;

    assert!(row.is_err());

    assert_eq!(403, create_user_response.status().as_u16());
}
// ---------------------------------------------------------------------------------------------------------------
// Original 422 status code is handled and converted to a 400 status code before response is sent to client
#[tokio::test]
async fn create_user_missing_fields_are_rejected() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);
    let url = format!("{}/v1/admin/create-user", server.addr);

    // Payload (Uses 'Admin' test user credentials)
    let body = json!({
        "email": server.test_users[1].email,
        "password": server.test_users[1].password
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

    // Payloads where the user should not be created
    let test_cases = vec![
        (
            json!({
                "email": "test@gmail.com",
                "password": "testing123",
                "role": "Reviewer",
            }),
            "missing name.",
        ),
        (
            json!({
                "name": "John",
                "password": "testing123",
                "role": "Reviewer",
            }),
            "missing email.",
        ),
        (
            json!({
                "name": "John",
                "email": "test@gmail.com",
                "role": "Reviewer",
            }),
            "missing password.",
        ),
        (
            json!({
                "name": "John",
                "email": "test@gmail.com",
                "password": "testing123",
            }),
            "missing role.",
        ),
    ];

    // 1. Create User Requests
    for (invalid_body, error_message) in test_cases {
        let response = server
            .post_create_user(&url, invalid_body.to_string(), &session_id.unwrap())
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
async fn create_user_returns_400_status() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);
    let url = format!("{}/v1/admin/create-user", server.addr);

    // Payload (Uses 'Admin' test user credentials)
    let body = json!({
        "email": server.test_users[1].email,
        "password": server.test_users[1].password
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

    // Payloads where the user should not be created
    let test_cases = vec![
        (
            json!({
                "name": "",
                "email": "test@gmail.com",
                "password": "testing123",
                "role": "Reviewer",
            }),
            "empty name",
        ),
        (
            json!({
                "name": "John",
                "email": "",
                "password": "testing123",
                "role": "Reviewer",
            }),
            "empty email",
        ),
        (
            json!({
                "name": "John",
                "email": "test@gmail.com",
                "password": "",
                "role": "Reviewer",
            }),
            "empty password",
        ),
        (
            json!({
                "name": "John",
                "email": "//$(test@gmail.com)",
                "password": "testing123",
                "role": "Reviewer",
            }),
            "malformed email",
        ),
        // Would return a 422 status because role is from the UserRole enum
        (
            json!({
                "name": "John",
                "email": "test@gmail.com",
                "password": "testing123",
                "role": "",
            }),
            "empty role",
        ),
    ];

    // 2. Create User Requests
    for (invalid_body, error_message) in test_cases {
        let response = server
            .post_create_user(&url, invalid_body.to_string(), &session_id.unwrap())
            .await;

        (
            assert_eq!(400, response.status().as_u16()),
            "API did not fail with a 400 status when the payload was {}",
            error_message,
        );
    }
}
