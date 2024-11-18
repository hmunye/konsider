use reqwest::header;
use serde_json::json;

use crate::common::{spawn_server, Result};

#[tokio::test]
async fn create_user_successful() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);
    let users_url = format!("{}/api/v1/users", server.addr);

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

    let create_user_body = json!({
        "name": "John",
        "email": "john@gmail.com",
        "password": "password1234",
        "role": "REVIEWER",
    });

    let create_user_response = server
        .post_request(
            &users_url,
            Some(create_user_body.to_string()),
            Some(&token.unwrap()),
        )
        .await?;
    assert_eq!(201, create_user_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn create_user_with_existing_email_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);
    let users_url = format!("{}/api/v1/users", server.addr);

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

    let create_user_body = json!({
        "name": "John",
        "email": &server.test_users[0].email,
        "password": "password1234",
        "role": "REVIEWER",
    });

    let create_user_response = server
        .post_request(
            &users_url,
            Some(create_user_body.to_string()),
            Some(&token.unwrap()),
        )
        .await?;
    assert_eq!(409, create_user_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn create_user_using_invalid_role_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);
    let users_url = format!("{}/api/v1/users", server.addr);

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

    let create_user_body = json!({
        "name": "John",
        "email": "john@gmail.com",
        "password": "password1234",
        "role": "REVIEWER",
    });

    let create_user_response = server
        .post_request(
            &users_url,
            Some(create_user_body.to_string()),
            Some(&token.unwrap()),
        )
        .await?;
    assert_eq!(403, create_user_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn create_user_with_invalid_fields_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);
    let users_url = format!("{}/api/v1/users", server.addr);

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
                "name": "",
                "email": "test@gmail.com",
                "password": "testing123456",
                "role": "REVIEWER"
            }),
            "empty name",
        ),
        (
            json!({
                "name": "John",
                "email": "",
                "password": "testing123456",
                "role": "REVIEWER"
            }),
            "empty email",
        ),
        (
            json!({
                "name": "John",
                "email": "test@gmail.com",
                "password": "",
                "role": "REVIEWER"
            }),
            "empty password",
        ),
        (
            json!({
                "name": "John",
                "email": "test@gmail.com",
                "password": "testing123456",
                "role": ""
            }),
            "empty role",
        ),
        (
            json!({
                "name": "//John$)",
                "email": "test@gmail.com",
                "password": "testing123456",
                "role": "REVIEWER"
            }),
            "malformed name",
        ),
        (
            json!({
                "name": "John",
                "email": "//$(test@gmail.com)",
                "password": "testing123456",
                "role": "REVIEWER"
            }),
            "malformed email",
        ),
        (
            json!({
                "name": "John",
                "email": "test@gmail.com",
                "password": "//John$)232343",
                "role": "REVIEWER"
            }),
            "malformed password",
        ),
        (
            json!({
                "name": "John",
                "email": "test@gmail.com",
                "password": "testing123456",
                "role": "R($ev\"iewer"
            }),
            "malformed role",
        ),
        (
            json!({
                "name": "John",
                "email": "testgmail.com",
                "password": "testing123456",
                "role": "REVIEWER"
            }),
            "invalid email",
        ),
        (
            json!({
                "name": "John",
                "email": "test@gmail.com",
                "password": "t",
                "role": "REVIEWER"
            }),
            "invalid password",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let create_user_response = server
            .post_request(
                &users_url,
                Some(invalid_body.to_string()),
                Some(&token.unwrap()),
            )
            .await?;
        (
            assert_eq!(400, create_user_response.status().as_u16()),
            "API did not fail with a 400 status when the payload was {}",
            error_message,
        );
    }

    Ok(())
}

#[tokio::test]
async fn create_user_with_missing_fields_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);
    let users_url = format!("{}/api/v1/users", server.addr);

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
                "email": "test@gmail.com",
                "password": "testing123",
                "role": "REVIEWER"
            }),
            "missing name",
        ),
        (
            json!({
                "name": "John",
                "password": "testing123",
                "role": "REVIEWER"
            }),
            "missing email",
        ),
        (
            json!({
                "name": "John",
                "email": "test@gmail.com",
                "role": "REVIEWER"
            }),
            "missing password",
        ),
        (
            json!({
                "name": "John",
                "email": "test@gmail.com",
                "password": "testing123"
            }),
            "missing role",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let create_user_response = server
            .post_request(
                &users_url,
                Some(invalid_body.to_string()),
                Some(&token.unwrap()),
            )
            .await?;
        (
            assert_eq!(400, create_user_response.status().as_u16()),
            "API did not fail with a 400 status when the payload was {}",
            error_message,
        );
    }

    Ok(())
}
