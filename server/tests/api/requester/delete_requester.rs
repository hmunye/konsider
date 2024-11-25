use reqwest::header;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::common::{spawn_server, Result};

#[tokio::test]
async fn delete_requester_successful() -> Result<()> {
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

    let get_requester_url = format!("{}/api/v1/requesters?filter=name:{}", server.addr, "John");

    let get_requester_response = server
        .get_request(&get_requester_url, Some(&token.unwrap()))
        .await?;
    assert_eq!(200, get_requester_response.status().as_u16());

    let requesters = get_requester_response.text().await.unwrap();

    let parsed_requester: Value = serde_json::from_str(&requesters).unwrap();

    let requester = &parsed_requester["requesters"][0]["requester"];

    let requester_id = requester["id"].as_str().unwrap_or_default().to_string();

    let delete_requester_response = server
        .delete_request(
            &format!("{}/{}", requesters_url, requester_id),
            Some(&token.unwrap()),
        )
        .await?;
    assert_eq!(204, delete_requester_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn delete_requester_using_missing_token_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let requesters_url = format!("{}/api/v1/requesters/{}", server.addr, Uuid::new_v4());

    let delete_requester_response = server.delete_request(&requesters_url, None).await?;
    assert_eq!(401, delete_requester_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn delete_requester_with_invalid_id_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);
    let requesters_url = format!("{}/api/v1/requesters/{}", server.addr, Uuid::new_v4());

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

    let delete_requester_response = server
        .delete_request(&requesters_url, Some(&token.unwrap()))
        .await?;
    // Returns a 404 status code to indicate the user does not exist
    assert_eq!(404, delete_requester_response.status().as_u16());

    Ok(())
}
