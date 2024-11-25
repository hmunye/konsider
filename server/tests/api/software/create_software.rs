use reqwest::header;
use serde_json::json;

use crate::common::{spawn_server, Result};

#[tokio::test]
async fn create_software_successful() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);
    let software_url = format!("{}/api/v1/software", server.addr);

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

    let create_software_body = json!({
        "software_name": "Test Software",
        "software_version": "1.0.0",
        "developer_name": "Test Developer",
        "description": "A test software application",
    });

    let create_software_response = server
        .post_request(
            &software_url,
            Some(create_software_body.to_string()),
            Some(&token.unwrap()),
        )
        .await?;
    assert_eq!(201, create_software_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn create_software_with_existing_name_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);
    let software_url = format!("{}/api/v1/software", server.addr);

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

    let create_software_body = json!({
        "software_name": "Test Software",
        "software_version": "1.0.0",
        "developer_name": "Test Developer",
        "description": "A test software application",
    });

    let create_software_response = server
        .post_request(
            &software_url,
            Some(create_software_body.to_string()),
            Some(&token.unwrap()),
        )
        .await?;
    assert_eq!(201, create_software_response.status().as_u16());

    let dup_create_software_response = server
        .post_request(
            &software_url,
            Some(create_software_body.to_string()),
            Some(&token.unwrap()),
        )
        .await?;
    assert_eq!(409, dup_create_software_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn create_software_using_missing_token_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let software_url = format!("{}/api/v1/software", server.addr);

    let create_software_body = json!({
        "software_name": "Test Software",
        "software_version": "1.0.0",
        "developer_name": "Test Developer",
        "description": "A test software application",
    });

    let create_software_response = server
        .post_request(&software_url, Some(create_software_body.to_string()), None)
        .await?;
    assert_eq!(401, create_software_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn create_software_with_invalid_fields_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);
    let software_url = format!("{}/api/v1/software", server.addr);

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
                "software_name": "",
                "software_version": "1.0.0",
                "developer_name": "Test Developer",
                "description": "A test software application",
            }),
            "empty software name",
        ),
        (
            json!({
                "software_name": "Test Software",
                "software_version": "",
                "developer_name": "Test Developer",
                "description": "A test software application",
            }),
            "empty software version",
        ),
        (
            json!({
                "software_name": "Test Software",
                "software_version": "1.0.0",
                "developer_name": "",
                "description": "A test software application",
            }),
            "empty developer name",
        ),
        (
            json!({
                "software_name": "Malformed Software!@#-",
                "software_version": "1.0.0",
                "developer_name": "Test Developer",
                "description": "A test software application",
            }),
            "malformed software name",
        ),
        (
            json!({
                "software_name": "Test Software",
                "software_version": "1.0.x",
                "developer_name": "Test Developer",
                "description": "A test software application",
            }),
            "malformed software version",
        ),
        (
            json!({
                "software_name": "Test Software",
                "software_version": "1.0.0",
                "developer_name": "Test Developer",
                "description": "Invalid characters in description: @$#",
            }),
            "malformed description",
        ),
        (
            json!({
                "software_name": "Test Software",
                "software_version": "1.0.0",
                "developer_name": "Test Developer",
                "description": "",
            }),
            "empty description",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let create_software_response = server
            .post_request(
                &software_url,
                Some(invalid_body.to_string()),
                Some(&token.unwrap()),
            )
            .await?;
        (assert_eq!(
            400,
            create_software_response.status().as_u16(),
            "API did not fail with a 400 status when the payload was {}",
            error_message,
        ),);
    }

    Ok(())
}

#[tokio::test]
async fn create_software_with_missing_fields_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);
    let software_url = format!("{}/api/v1/software", server.addr);

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
                "software_version": "1.0.0",
                "developer_name": "Test Developer",
                "description": "A test software application",
            }),
            "missing software name",
        ),
        (
            json!({
                "software_name": "Test Software",
                "developer_name": "Test Developer",
                "description": "A test software application",
            }),
            "missing software version",
        ),
        (
            json!({
                "software_name": "Test Software",
                "software_version": "1.0.0",
                "description": "A test software application",
            }),
            "missing developer name",
        ),
        (
            json!({
                "software_name": "Test Software",
                "software_version": "1.0.0",
                "developer_name": "Test Developer",
            }),
            "missing description",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let create_software_response = server
            .post_request(
                &software_url,
                Some(invalid_body.to_string()),
                Some(&token.unwrap()),
            )
            .await?;
        (assert_eq!(
            400,
            create_software_response.status().as_u16(),
            "API did not fail with a 400 status when the payload was {}",
            error_message,
        ),);
    }

    Ok(())
}
