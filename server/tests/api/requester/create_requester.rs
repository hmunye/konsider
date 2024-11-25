use reqwest::header;
use serde_json::json;

use crate::common::{spawn_server, Result};

#[tokio::test]
async fn create_requester_successful() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);
    let requesters_url = format!("{}/api/v1/requesters", server.addr);

    // Uses 'Admin' test user credentials
    let login_body = json!({
        "email": server.test_users[1].email,
        "password": server.test_users[1].password
    });

    let login_response = server
        .post_request(&login_url, Some(login_body.to_string()), None)
        .await?;
    assert_eq!(200, login_response.status().as_u16());

    let token = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok());
    assert!(token.is_some(), "JWT should be present");

    let create_requester_body = json!({
        "name": "John",
        "email": "john@gmail.com",
        "department": "IT",
    });

    let create_requester_response = server
        .post_request(
            &requesters_url,
            Some(create_requester_body.to_string()),
            Some(&token.unwrap()),
        )
        .await?;
    assert_eq!(201, create_requester_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn create_requester_with_existing_email_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);
    let requesters_url = format!("{}/api/v1/requesters", server.addr);

    // Uses 'Admin' test user credentials
    let login_body = json!({
        "email": server.test_users[1].email,
        "password": server.test_users[1].password
    });

    let login_response = server
        .post_request(&login_url, Some(login_body.to_string()), None)
        .await?;
    assert_eq!(200, login_response.status().as_u16());

    let token = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok());
    assert!(token.is_some(), "JWT should be present");

    let create_requester_body = json!({
        "name": "John",
        "email": "john@gmail.com",
        "department": "IT",
    });

    let create_requester_response = server
        .post_request(
            &requesters_url,
            Some(create_requester_body.to_string()),
            Some(&token.unwrap()),
        )
        .await?;
    assert_eq!(201, create_requester_response.status().as_u16());

    let dup_create_requester_response = server
        .post_request(
            &requesters_url,
            Some(create_requester_body.to_string()),
            Some(&token.unwrap()),
        )
        .await?;
    assert_eq!(409, dup_create_requester_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn create_requester_using_missing_token_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let requesters_url = format!("{}/api/v1/requesters", server.addr);

    let create_requester_body = json!({
        "name": "John",
        "email": "john@gmail.com",
        "department": "IT",
    });

    let create_requester_response = server
        .post_request(
            &requesters_url,
            Some(create_requester_body.to_string()),
            None,
        )
        .await?;
    assert_eq!(401, create_requester_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn create_requester_with_invalid_fields_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);
    let requesters_url = format!("{}/api/v1/requesters", server.addr);

    // Uses 'Admin' test user credentials
    let login_body = json!({
        "email": server.test_users[1].email,
        "password": server.test_users[1].password
    });

    let login_response = server
        .post_request(&login_url, Some(login_body.to_string()), None)
        .await?;
    assert_eq!(200, login_response.status().as_u16());

    let token = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok());
    assert!(token.is_some(), "JWT should be present");

    let test_cases = vec![
        (
            json!({
                "name": "",
                "email": "john@gmail.com",
                "department": "IT",
            }),
            "empty name",
        ),
        (
            json!({
                "name": "John",
                "email": "",
                "department": "IT",
            }),
            "empty email",
        ),
        (
            json!({
                "name": "John",
                "email": "test@gmail.com",
                "department": "",
            }),
            "empty department",
        ),
        (
            json!({
                "name": "//John$)",
                "email": "john@gmail.com",
                "department": "IT",
            }),
            "malformed name",
        ),
        (
            json!({
                "name": "John",
                "email": "//$(test@gmail.com)",
                "department": "IT",
            }),
            "malformed email",
        ),
        (
            json!({
                "name": "John",
                "email": "test@gmail.com",
                "department": "--$(IT\'/",
            }),
            "malformed department",
        ),
        (
            json!({
                "name": "John",
                "email": "testgmail.com",
                "department": "IT",
            }),
            "invalid email",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let create_requester_response = server
            .post_request(
                &requesters_url,
                Some(invalid_body.to_string()),
                Some(&token.unwrap()),
            )
            .await?;
        (assert_eq!(
            400,
            create_requester_response.status().as_u16(),
            "API did not fail with a 400 status when the payload was {}",
            error_message,
        ),);
    }

    Ok(())
}

#[tokio::test]
async fn create_requester_with_missing_fields_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);
    let requesters_url = format!("{}/api/v1/requesters", server.addr);

    // Uses 'Admin' test user credentials
    let login_body = json!({
        "email": server.test_users[1].email,
        "password": server.test_users[1].password
    });

    let login_response = server
        .post_request(&login_url, Some(login_body.to_string()), None)
        .await?;
    assert_eq!(200, login_response.status().as_u16());

    let token = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok());
    assert!(token.is_some(), "JWT should be present");

    let test_cases = vec![
        (
            json!({
                "email": "john@gmail.com",
                "department": "IT",
            }),
            "missing name",
        ),
        (
            json!({
                "name": "John",
                "department": "IT",
            }),
            "missing email",
        ),
        (
            json!({
                "name": "John",
                "email": "test@gmail.com",
            }),
            "missing department",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let create_requester_response = server
            .post_request(
                &requesters_url,
                Some(invalid_body.to_string()),
                Some(&token.unwrap()),
            )
            .await?;
        (assert_eq!(
            400,
            create_requester_response.status().as_u16(),
            "API did not fail with a 400 status when the payload was {}",
            error_message,
        ),);
    }

    Ok(())
}
