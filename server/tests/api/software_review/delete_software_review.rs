use reqwest::header;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::common::{spawn_server, Result};

#[tokio::test]
async fn delete_software_review_successful() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);
    let software_review_url = format!("{}/api/v1/reviews", server.addr);

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

    let create_software_review_body = json!({
        "software_request": {
            "td_request_id": "12345678",
            "software": {
                "software_name": "Test Software",
                "software_version": "1.0.0",
                "developer_name": "Test Developer",
                "description": "A test software application",
            },
            "requester": {
                "name": "John",
                "email": "john@gmail.com",
                "department": "IT",
            },
        },
        "reviewer_id": server.test_users[1].id,
        "is_supported": "TRUE",
        "is_current_version": "TRUE",
        "is_reputation_good": "TRUE",
        "is_installation_from_developer": "TRUE",
        "is_local_admin_required": "FALSE",
        "is_connected_to_brockport_cloud": "FALSE",
        "is_connected_to_cloud_services_or_client": "FALSE",
        "is_security_or_optimization_software": "FALSE",
        "is_supported_by_current_os": "TRUE",
        "review_notes": "All conditions satisfied."
    });

    let create_software_review_response = server
        .post_request(
            &software_review_url,
            Some(create_software_review_body.to_string()),
            Some(&token.unwrap()),
        )
        .await?;
    assert_eq!(201, create_software_review_response.status().as_u16());

    let get_software_review_url = format!(
        "{}?filter=td_request_id:{}",
        software_review_url, "12345678"
    );

    let get_software_review_response = server
        .get_request(&get_software_review_url, Some(&token.unwrap()))
        .await?;
    assert_eq!(200, get_software_review_response.status().as_u16());

    let software_reviews = get_software_review_response.text().await.unwrap();

    let parsed_software_review: Value = serde_json::from_str(&software_reviews).unwrap();

    let software_review = &parsed_software_review["software_reviews"][0]["software_review"];

    let software_review_id = software_review["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    let delete_software_review_response = server
        .delete_request(
            &format!("{}/{}", software_review_url, software_review_id),
            Some(&token.unwrap()),
        )
        .await?;
    assert_eq!(204, delete_software_review_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn delete_software_request_using_missing_token_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let software_review_url = format!("{}/api/v1/reviews/{}", server.addr, Uuid::new_v4());

    let delete_software_review_response = server.delete_request(&software_review_url, None).await?;
    assert_eq!(401, delete_software_review_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn delete_software_with_invalid_id_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);
    let software_review_url = format!("{}/api/v1/reviews/{}", server.addr, Uuid::new_v4());

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

    let delete_software_review_response = server
        .delete_request(&software_review_url, Some(&token.unwrap()))
        .await?;
    // Returns a 404 status code to indicate the user does not exist
    assert_eq!(404, delete_software_review_response.status().as_u16());

    Ok(())
}
