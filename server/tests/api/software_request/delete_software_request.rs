use reqwest::header;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::common::{spawn_server, Result};

#[tokio::test]
async fn delete_software_request_successful() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);
    let software_url = format!("{}/api/v1/software", server.addr);
    let requesters_url = format!("{}/api/v1/requesters", server.addr);
    let software_request_url = format!("{}/api/v1/requests", server.addr);

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

    let get_requester_url = format!("{}/api/v1/requesters?filter=name:{}", server.addr, "John");

    let get_requester_response = server
        .get_request(&get_requester_url, Some(&token.unwrap()))
        .await?;
    assert_eq!(200, get_requester_response.status().as_u16());

    let requesters = get_requester_response.text().await.unwrap();

    let parsed_requester: Value = serde_json::from_str(&requesters).unwrap();

    let requester = &parsed_requester["requesters"][0]["requester"];

    let requester_id = requester["id"].as_str().unwrap_or_default().to_string();

    let create_software_request_body = json!({
        "td_request_id": "12345678",
        "software_id": software_id,
        "requester_id": requester_id
    });

    let create_software_request_response = server
        .post_request(
            &software_request_url,
            Some(create_software_request_body.to_string()),
            Some(&token.unwrap()),
        )
        .await?;
    assert_eq!(201, create_software_request_response.status().as_u16());

    let get_software_request_url = format!(
        "{}?filter=td_request_id:{}",
        software_request_url, "12345678"
    );

    let get_software_request_response = server
        .get_request(&get_software_request_url, Some(&token.unwrap()))
        .await?;
    assert_eq!(200, get_software_request_response.status().as_u16());

    let software_requests = get_software_request_response.text().await.unwrap();

    let parsed_software_request: Value = serde_json::from_str(&software_requests).unwrap();

    let software_request = &parsed_software_request["software_requests"][0]["software_request"];

    let software_request_id = software_request["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    let delete_software_request_response = server
        .delete_request(
            &format!("{}/{}", software_request_url, software_request_id),
            Some(&token.unwrap()),
        )
        .await?;
    assert_eq!(204, delete_software_request_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn delete_software_request_using_missing_token_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let software_request_url = format!("{}/api/v1/requests/{}", server.addr, Uuid::new_v4());

    let delete_software_request_response =
        server.delete_request(&software_request_url, None).await?;
    assert_eq!(401, delete_software_request_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn delete_software_with_invalid_id_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);
    let software_request_url = format!("{}/api/v1/requests/{}", server.addr, Uuid::new_v4());

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

    let delete_software_request_response = server
        .delete_request(&software_request_url, Some(&token.unwrap()))
        .await?;
    // Returns a 404 status code to indicate the user does not exist
    assert_eq!(404, delete_software_request_response.status().as_u16());

    Ok(())
}
