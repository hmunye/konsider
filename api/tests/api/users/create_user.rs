use reqwest::header;
use serde_json::json;
use uuid::Uuid;

use crate::common::spawn_server;

// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn create_user_successful() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);
    let users_url = format!("{}/v1/users", server.addr);

    // Uses 'Admin' test user credentials
    let body = json!({
        "email": server.test_users[1].email,
        "password": server.test_users[1].password
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

    let email: String = String::from("john@gmail.com");

    let body = json!({
        "user": {
            "name": "John",
            "email": &email,
            "password": "password1234",
            "role": "Reviewer",
        },
        "idempotency_key": Uuid::new_v4().to_string()
    });

    let create_user_response = server
        .post_request(
            &users_url,
            Some(body.to_string()),
            Some(&session_id.unwrap()),
            None,
        )
        .await;
    assert_eq!(201, create_user_response.status().as_u16());

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
async fn create_user_with_existing_email_rejected() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);
    let users_url = format!("{}/v1/users", server.addr);

    // Uses 'Admin' test user credentials
    let body = json!({
        "email": server.test_users[1].email,
        "password": server.test_users[1].password
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

    let email = server.test_users[1].email.clone();

    let body = json!({
        "user": {
            "name": "John",
            "email": &email,
            "password": "password1234",
            "role": "Reviewer",
        },
        "idempotency_key": Uuid::new_v4().to_string()
    });

    let create_user_response = server
        .post_request(
            &users_url,
            Some(body.to_string()),
            Some(&session_id.unwrap()),
            None,
        )
        .await;
    assert_eq!(409, create_user_response.status().as_u16());
}
// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn create_user_using_invalid_role_rejected() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);
    let users_url = format!("{}/v1/users", server.addr);

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

    let email: String = String::from("john@gmail.com");

    let body = json!({
        "user": {
            "name": "John",
            "email": &email,
            "password": "password1234",
            "role": "Reviewer",
        },
        "idempotency_key": Uuid::new_v4().to_string()
    });

    let create_user_response = server
        .post_request(
            &users_url,
            Some(body.to_string()),
            Some(&session_id.unwrap()),
            None,
        )
        .await;
    assert_eq!(403, create_user_response.status().as_u16());
}
// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn create_user_with_invalid_fields_rejected() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);
    let users_url = format!("{}/v1/users", server.addr);

    // Uses 'Admin' test user credentials
    let body = json!({
    "email": server.test_users[1].email,
    "password": server.test_users[1].password
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

    let test_cases = vec![
        (
            json!({
                "user": {
                    "name": "",
                    "email": "test@gmail.com",
                    "password": "testing123456",
                    "role": "Reviewer"
                },
                "idempotency_key": Uuid::new_v4().to_string()
            }),
            "empty name",
        ),
        (
            json!({
                "user": {
                    "name": "John",
                    "email": "",
                    "password": "testing123456",
                    "role": "Reviewer"
                },
                "idempotency_key": Uuid::new_v4().to_string()
            }),
            "empty email",
        ),
        (
            json!({
                "user": {
                    "name": "John",
                    "email": "test@gmail.com",
                    "password": "",
                    "role": "Reviewer"
                },
                "idempotency_key": Uuid::new_v4().to_string()
            }),
            "empty password",
        ),
        (
            json!({
                "user": {
                    "name": "John",
                    "email": "test@gmail.com",
                    "password": "testing123456",
                    "role": ""
                },
                "idempotency_key": Uuid::new_v4().to_string()
            }),
            "empty role",
        ),
        (
            json!({
                "user": {
                    "name": "John",
                    "email": "test@gmail.com",
                    "password": "testing123456",
                    "role": "Reviewer"
                },
                "idempotency_key": ""
            }),
            "empty idempotency key",
        ),
        (
            json!({
                "user": {
                    "name": "//John$)",
                    "email": "test@gmail.com",
                    "password": "testing123456",
                    "role": "Reviewer"
                },
                "idempotency_key": Uuid::new_v4().to_string()
            }),
            "malformed name",
        ),
        (
            json!({
                "user": {
                    "name": "John",
                    "email": "//$(test@gmail.com)",
                    "password": "testing123456",
                    "role": "Reviewer"
                },
                "idempotency_key": Uuid::new_v4().to_string()
            }),
            "malformed email",
        ),
        (
            json!({
                "user": {
                    "name": "John",
                    "email": "test@gmail.com",
                    "password": "//John$)232343",
                    "role": "Reviewer"
                },
                "idempotency_key": Uuid::new_v4().to_string()
            }),
            "malformed password",
        ),
        (
            json!({
                "user": {
                    "name": "John",
                    "email": "test@gmail.com",
                    "password": "testing123456",
                    "role": "R($ev\"iewer"
                },
                "idempotency_key": Uuid::new_v4().to_string()
            }),
            "malformed role",
        ),
        (
            json!({
                "user": {
                    "name": "John",
                    "email": "testgmail.com",
                    "password": "testing123456",
                    "role": "Reviewer"
                },
                "idempotency_key": Uuid::new_v4().to_string()
            }),
            "invalid email",
        ),
        (
            json!({
                "user": {
                    "name": "John",
                    "email": "test@gmail.com",
                    "password": "t",
                    "role": "Reviewer"
                },
                "idempotency_key": Uuid::new_v4().to_string()
            }),
            "invalid password",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = server
            .post_request(
                &users_url,
                Some(invalid_body.to_string()),
                Some(&session_id.unwrap()),
                None,
            )
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
async fn create_user_with_missing_fields_rejected() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);
    let users_url = format!("{}/v1/users", server.addr);

    // Uses 'Admin' test user credentials
    let body = json!({
        "email": server.test_users[1].email,
        "password": server.test_users[1].password
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

    let test_cases = vec![
        (
            json!({
                "user": {
                    "email": "test@gmail.com",
                    "password": "testing123",
                    "role": "Reviewer"
                },
                "idempotency_key": Uuid::new_v4().to_string()
            }),
            "missing name",
        ),
        (
            json!({
                "user": {
                    "name": "John",
                    "password": "testing123",
                    "role": "Reviewer"
                },
                "idempotency_key": Uuid::new_v4().to_string()
            }),
            "missing email",
        ),
        (
            json!({
                "user": {
                    "name": "John",
                    "email": "test@gmail.com",
                    "role": "Reviewer"
                },
                "idempotency_key": Uuid::new_v4().to_string()
            }),
            "missing password",
        ),
        (
            json!({
                "user": {
                    "name": "John",
                    "email": "test@gmail.com",
                    "password": "testing123"
                },
                "idempotency_key": Uuid::new_v4().to_string()
            }),
            "missing role",
        ),
        (
            json!({
                "user": {
                    "name": "John",
                    "email": "test@gmail.com",
                    "password": "testing123",
                    "role": "Reviewer"
                },
            }),
            "missing idempotency key",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = server
            .post_request(
                &users_url,
                Some(invalid_body.to_string()),
                Some(&session_id.unwrap()),
                None,
            )
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
async fn create_user_is_idempotent() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);
    let users_url = format!("{}/v1/users", server.addr);

    // Uses 'Admin' test user credentials
    let body = json!({
        "email": server.test_users[1].email,
        "password": server.test_users[1].password
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

    let email: String = String::from("john@gmail.com");

    let body = json!({
        "user": {
            "name": "John",
            "email": &email,
            "password": "password1234",
            "role": "Reviewer",
        },
        "idempotency_key": Uuid::new_v4().to_string()
    });

    let create_user_response = server
        .post_request(
            &users_url,
            Some(body.to_string()),
            Some(&session_id.unwrap()),
            None,
        )
        .await;
    assert_eq!(201, create_user_response.status().as_u16());

    let dup_create_user_response = server
        .post_request(
            &users_url,
            Some(body.to_string()),
            Some(&session_id.unwrap()),
            None,
        )
        .await;
    assert_eq!(204, dup_create_user_response.status().as_u16());
}
