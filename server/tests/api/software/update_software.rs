use reqwest::header;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::common::{spawn_server, Result};

#[tokio::test]
async fn update_software_successful() -> Result<()> {
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
    assert_eq!(204, login_response.status().as_u16());

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

    let get_software_url = format!(
        "{}/api/v1/software?filter=software_name:{}",
        server.addr, "Test Software"
    );

    let get_software_response = server
        .get_request(&get_software_url, Some(&token.unwrap()))
        .await?;
    assert_eq!(200, get_software_response.status().as_u16());

    let software = get_software_response.text().await.unwrap();

    let parsed_software: Value = serde_json::from_str(&software).unwrap();

    let software = &parsed_software["software"][0]["software"];

    let software_id = software["id"].as_str().unwrap_or_default().to_string();

    let original_software = vec![
        software["software_name"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        software["software_version"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        software["developer_name"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        software["description"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
    ];

    let test_cases = vec![
        json!({
            "software_name": "New Software",
            "software_version": "2.0.1",
            "developer_name": "John Developer",
            "description": "An updated software description",
        }),
        json!({
            "software_name": "Another Software",
        }),
        json!({
            "software_version": "3.0.0",
        }),
        json!({
            "developer_name": "Alice Developer",
        }),
        json!({
            "description": "A description of the software",
        }),
    ];

    for valid_body in test_cases {
        let update_software_response = server
            .patch_request(
                &format!("{}/{}", software_url, software_id),
                Some(valid_body.to_string()),
                Some(&token.unwrap()),
            )
            .await?;
        (
            assert_eq!(204, update_software_response.status().as_u16()),
            "API did not succeed with a 204 status when the payload was {}",
            valid_body,
        );
    }

    let get_software_url = format!(
        "{}/api/v1/software?filter=software_name:{}",
        server.addr, "Another Software"
    );

    let get_software_response = server
        .get_request(&get_software_url, Some(&token.unwrap()))
        .await?;
    assert_eq!(200, get_software_response.status().as_u16());

    let software = get_software_response.text().await.unwrap();

    let parsed_software: Value = serde_json::from_str(&software).unwrap();

    let software = &parsed_software["software"][0]["software"];

    let updated_software = vec![
        software["software_name"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        software["software_version"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        software["developer_name"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        software["description"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
    ];

    assert!(!(original_software == updated_software));

    Ok(())
}

#[tokio::test]
async fn update_software_using_missing_token_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let software_url = format!("{}/api/v1/software/{}", server.addr, Uuid::new_v4());

    let update_software_body = json!({
        "software_name": "New Software",
        "software_version": "2.0.1",
        "developer_name": "John Developer",
        "description": "An updated software description",
    });

    let update_software_response = server
        .patch_request(&software_url, Some(update_software_body.to_string()), None)
        .await?;
    assert_eq!(401, update_software_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn update_software_with_invalid_id_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);
    let software_url = format!("{}/api/v1/software/{}", server.addr, Uuid::new_v4());

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

    let update_software_body = json!({
        "software_name": "New Software",
        "software_version": "2.0.1",
        "developer_name": "John Developer",
        "description": "An updated software description",
    });

    let update_software_response = server
        .patch_request(
            &software_url,
            Some(update_software_body.to_string()),
            Some(&token.unwrap()),
        )
        .await?;
    // Returns a 404 status code to indicate the software does not exist
    assert_eq!(404, update_software_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn update_software_with_invalid_fields_rejected() -> Result<()> {
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
    assert_eq!(204, login_response.status().as_u16());

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

    let get_software_url = format!(
        "{}/api/v1/software?filter=software_name:{}",
        server.addr, "Test Software"
    );

    let get_software_response = server
        .get_request(&get_software_url, Some(&token.unwrap()))
        .await?;
    assert_eq!(200, get_software_response.status().as_u16());

    let software = get_software_response.text().await.unwrap();

    let parsed_software: Value = serde_json::from_str(&software).unwrap();

    let software = &parsed_software["software"][0]["software"];

    let software_id = software["id"].as_str().unwrap_or_default().to_string();

    let test_cases = vec![
        (
            json!({
                "software_name": "Invalid!@#Name-",
            }),
            "invalid software name",
        ),
        (
            json!({
                "software_version": "1.0.a",
            }),
            "invalid software version",
        ),
        (
            json!({
                "developer_name": "John@Dev$-",
            }),
            "invalid developer name",
        ),
        (
            json!({
                "description": "Invalid description@@!-",
            }),
            "invalid description",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let update_software_response = server
            .patch_request(
                &format!("{}/{}", software_url, software_id),
                Some(invalid_body.to_string()),
                Some(&token.unwrap()),
            )
            .await?;
        (assert_eq!(
            400,
            update_software_response.status().as_u16(),
            "API did not fail with a 400 status when the payload was {}",
            error_message,
        ),);
    }

    Ok(())
}

#[tokio::test]
async fn update_software_with_missing_fields_rejected() -> Result<()> {
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
    assert_eq!(204, login_response.status().as_u16());

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

    let get_software_url = format!(
        "{}/api/v1/software?filter=software_name:{}",
        server.addr, "Test Software"
    );

    let get_software_response = server
        .get_request(&get_software_url, Some(&token.unwrap()))
        .await?;
    assert_eq!(200, get_software_response.status().as_u16());

    let software = get_software_response.text().await.unwrap();

    let parsed_software: Value = serde_json::from_str(&software).unwrap();

    let software = &parsed_software["software"][0]["software"];

    let software_id = software["id"].as_str().unwrap_or_default().to_string();

    let update_software_body = json!({});

    let update_software_response = server
        .patch_request(
            &format!("{}/{}", software_url, software_id),
            Some(update_software_body.to_string()),
            Some(&token.unwrap()),
        )
        .await?;
    assert_eq!(400, update_software_response.status().as_u16());

    Ok(())
}
