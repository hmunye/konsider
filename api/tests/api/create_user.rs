use reqwest::header;
use serde_json::json;

use crate::common::spawn_server;

// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn create_user_successful() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);
    let create_user_url = format!("{}/v1/admin/create-user", server.addr);

    // Uses 'Admin' test user credentials
    let body = json!({
        "email": server.test_users[1].email,
        "password": server.test_users[1].password
    });

    let login_response = server
        .post_request(&login_url, Some(body.to_string()), None)
        .await;
    assert_eq!(200, login_response.status().as_u16());

    // TODO: Find out how to correctly preserve cookies without manual extraction
    let session_id = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|str| str.split(";").nth(0));

    let email: String = String::from("john@gmail.com");

    let body = json!({
        "name": "John",
        "email": &email,
        "password": "password1234",
        "role": "Reviewer"
    });

    let create_user_response = server
        .post_request(
            &create_user_url,
            Some(body.to_string()),
            Some(&session_id.unwrap()),
        )
        .await;
    assert_eq!(200, create_user_response.status().as_u16());

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
    let create_user_url = format!("{}/v1/admin/create-user", server.addr);

    // Uses 'Admin' test user credentials
    let body = json!({
        "email": server.test_users[1].email,
        "password": server.test_users[1].password
    });

    let login_response = server
        .post_request(&login_url, Some(body.to_string()), None)
        .await;
    assert_eq!(200, login_response.status().as_u16());

    // TODO: Find out how to correctly preserve cookies without manual extraction
    let session_id = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|str| str.split(";").nth(0));

    let email = server.test_users[1].email.clone();

    let body = json!({
        "name": "John",
        "email": &email,
        "password": "password1234",
        "role": "Reviewer"
    });

    let create_user_response = server
        .post_request(
            &create_user_url,
            Some(body.to_string()),
            Some(&session_id.unwrap()),
        )
        .await;
    assert_eq!(409, create_user_response.status().as_u16());
}
// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn create_user_using_invalid_role_rejected() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);
    let create_user_url = format!("{}/v1/admin/create-user", server.addr);

    // Uses 'Reviewer' test user credentials
    let body = json!({
        "email": server.test_users[0].email,
        "password": server.test_users[0].password
    });

    let login_response = server
        .post_request(&login_url, Some(body.to_string()), None)
        .await;
    assert_eq!(200, login_response.status().as_u16());

    // TODO: Find out how to correctly preserve cookies without manual extraction
    let session_id = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|str| str.split(";").nth(0));

    let email: String = String::from("john@gmail.com");

    let body = json!({
        "name": "John",
        "email": &email,
        "password": "password123",
        "role": "Reviewer"
    });

    let create_user_response = server
        .post_request(
            &create_user_url,
            Some(body.to_string()),
            Some(&session_id.unwrap()),
        )
        .await;
    assert_eq!(403, create_user_response.status().as_u16());
}
// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn create_user_with_invalid_fields_rejected() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);
    let create_user_url = format!("{}/v1/admin/create-user", server.addr);

    // Uses 'Admin' test user credentials
    let body = json!({
    "email": server.test_users[1].email,
    "password": server.test_users[1].password
    });

    let login_response = server
        .post_request(&login_url, Some(body.to_string()), None)
        .await;
    assert_eq!(200, login_response.status().as_u16());

    // TODO: Find out how to correctly preserve cookies without manual extraction
    let session_id = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|str| str.split(";").nth(0));

    let test_cases = vec![
        (
            json!({
            "name": "",
            "email": "test@gmail.com",
            "password": "testing123456",
            "role": "Reviewer",
            }),
            "empty name",
        ),
        (
            json!({
            "name": "John",
            "email": "",
            "password": "testing123456",
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
            "email": "test@gmail.com",
            "password": "testing123456",
            "role": "",
            }),
            "empty role",
        ),
        (
            json!({
            "name": "//John$)",
            "email": "test@gmail.com",
            "password": "testing123456",
            "role": "Reviewer",
            }),
            "malformed name",
        ),
        (
            json!({
            "name": "John",
            "email": "//$(test@gmail.com)",
            "password": "testing123456",
            "role": "Reviewer",
            }),
            "malformed email",
        ),
        (
            json!({
            "name": "John",
            "email": "test@gmail.com",
            "password": "//John$)232343",
            "role": "Reviewer",
            }),
            "malformed password",
        ),
        (
            json!({
            "name": "John",
            "email": "test@gmail.com",
            "password": "testing123456",
            "role": "R($ev\"iewer",
            }),
            "malformed role",
        ),
        (
            json!({
            "name": "John",
            "email": "testgmail.com",
            "password": "testing123456",
            "role": "Reviewer",
            }),
            "invaild email",
        ),
        (
            json!({
            "name": "John",
            "email": "test@gmail.com",
            "password": "t",
            "role": "Reviewer",
            }),
            "invaild password",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = server
            .post_request(
                &create_user_url,
                Some(invalid_body.to_string()),
                Some(&session_id.unwrap()),
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
    let create_user_url = format!("{}/v1/admin/create-user", server.addr);

    // Uses 'Admin' test user credentials
    let body = json!({
        "email": server.test_users[1].email,
        "password": server.test_users[1].password
    });

    let login_response = server
        .post_request(&login_url, Some(body.to_string()), None)
        .await;
    assert_eq!(200, login_response.status().as_u16());

    // TODO: Find out how to correctly preserve cookies without manual extraction
    let session_id = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|str| str.split(";").nth(0));

    let test_cases = vec![
        (
            json!({
                "email": "test@gmail.com",
                "password": "testing123",
                "role": "Reviewer",
            }),
            "missing name",
        ),
        (
            json!({
                "name": "John",
                "password": "testing123",
                "role": "Reviewer",
            }),
            "missing email",
        ),
        (
            json!({
                "name": "John",
                "email": "test@gmail.com",
                "role": "Reviewer",
            }),
            "missing password",
        ),
        (
            json!({
                "name": "John",
                "email": "test@gmail.com",
                "password": "testing123",
            }),
            "missing role",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = server
            .post_request(
                &create_user_url,
                Some(invalid_body.to_string()),
                Some(&session_id.unwrap()),
            )
            .await;

        (
            assert_eq!(400, response.status().as_u16()),
            "API did not fail with a 400 status when the payload was {}",
            error_message,
        );
    }
}
