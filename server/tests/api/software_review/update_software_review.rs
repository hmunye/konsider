use reqwest::header;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::common::{spawn_server, Result};

#[tokio::test]
async fn update_software_review_successful() -> Result<()> {
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

    let original_software_review = vec![
        software_review["is_supported"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        software_review["is_current_version"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        software_review["is_reputation_good"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        software_review["is_installation_from_developer"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        software_review["is_local_admin_required"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        software_review["is_connected_to_brockport_cloud"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        software_review["is_connected_to_cloud_services_or_client"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        software_review["is_security_or_optimization_software"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        software_review["is_supported_by_current_os"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        software_review["review_notes"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
    ];

    let software_review_id = software_review["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    let test_cases = vec![
        json!({
            "is_supported": "TRUE",
            "is_current_version": "TRUE",
            "is_reputation_good": "FALSE",
            "is_installation_from_developer": "TRUE",
            "is_local_admin_required": "FALSE",
            "is_connected_to_brockport_cloud": "TRUE",
            "is_connected_to_cloud_services_or_client": "FALSE",
            "is_security_or_optimization_software": "TRUE",
            "is_supported_by_current_os": "FALSE",
            "review_notes": "Review notes go here"
        }),
        json!({
            "is_supported": "TRUE",
            "is_current_version": "NOT_SURE",
            "review_notes": "Updated review notes"
        }),
        json!({
            "is_reputation_good": "TRUE",
            "is_local_admin_required": "NOT_SURE",
            "review_notes": "Mixed feelings about the software."
        }),
        json!({
            "is_supported": "NOT_SURE",
            "review_notes": "No issues with the software."
        }),
        json!({
            "is_supported": "FALSE",
            "is_current_version": "NOT_SURE",
            "is_reputation_good": "NOT_SURE",
            "is_local_admin_required": "TRUE",
            "review_notes": "Review contains uncertain aspects."
        }),
    ];

    for valid_body in test_cases {
        let update_software_review_response = server
            .patch_request(
                &format!("{}/{}", software_review_url, software_review_id),
                Some(valid_body.to_string()),
                Some(&token.unwrap()),
            )
            .await?;
        assert_eq!(204, update_software_review_response.status().as_u16());
        (
            assert_eq!(204, update_software_review_response.status().as_u16()),
            "API did not succeed with a 204 status when the payload was {}",
            valid_body,
        );
    }

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

    let updated_software_review = vec![
        software_review["is_supported"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        software_review["is_current_version"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        software_review["is_reputation_good"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        software_review["is_installation_from_developer"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        software_review["is_local_admin_required"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        software_review["is_connected_to_brockport_cloud"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        software_review["is_connected_to_cloud_services_or_client"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        software_review["is_security_or_optimization_software"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        software_review["is_supported_by_current_os"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        software_review["review_notes"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
    ];

    assert!(!(original_software_review == updated_software_review));

    Ok(())
}

#[tokio::test]
async fn update_software_review_using_missing_token_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let software_review_url = format!("{}/api/v1/reviews/{}", server.addr, Uuid::new_v4());

    let update_software_review_body = json!({
        "is_supported": "YES",
        "is_current_version": "YES",
        "is_reputation_good": "NO",
        "is_installation_from_developer": "YES",
        "is_local_admin_required": "NO",
        "is_connected_to_brockport_cloud": "YES",
        "is_connected_to_cloud_services_or_client": "NO",
        "is_security_or_optimization_software": "YES",
        "is_supported_by_current_os": "NO",
        "review_notes": "Review notes go here"
    });

    let update_software_review_response = server
        .patch_request(
            &software_review_url,
            Some(update_software_review_body.to_string()),
            None,
        )
        .await?;
    assert_eq!(401, update_software_review_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn update_software_review_with_invalid_id_rejected() -> Result<()> {
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

    let update_software_review_body = json!({
        "is_supported": "TRUE",
        "is_current_version": "TRUE",
        "is_reputation_good": "FALSE",
        "is_installation_from_developer": "TRUE",
        "is_local_admin_required": "FALSE",
        "is_connected_to_brockport_cloud": "TRUE",
        "is_connected_to_cloud_services_or_client": "FALSE",
        "is_security_or_optimization_software": "TRUE",
        "is_supported_by_current_os": "FALSE",
        "review_notes": "Review notes go here"
    });

    let update_software_request_response = server
        .patch_request(
            &software_review_url,
            Some(update_software_review_body.to_string()),
            Some(&token.unwrap()),
        )
        .await?;
    assert_eq!(404, update_software_request_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn update_software_review_with_invalid_fields_rejected() -> Result<()> {
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

    let test_cases = vec![
        (
            json!({
                "is_supported": "",
            }),
            "empty is_supported",
        ),
        (
            json!({
                "is_current_version": "maybe",
            }),
            "invalid is_current_version",
        ),
        (
            json!({
                "is_reputation_good": "YES",
            }),
            "invalid is_reputation_good",
        ),
        (
            json!({
                "is_installation_from_developer": "definitely",
            }),
            "invalid is_installation_from_developer",
        ),
        (
            json!({
                "is_local_admin_required": "yesplease",
            }),
            "invalid is_local_admin_required",
        ),
        (
            json!({
                "is_connected_to_brockport_cloud": "possibly",
            }),
            "invalid is_connected_to_brockport_cloud",
        ),
        (
            json!({
                "is_connected_to_cloud_services_or_client": "maybe",
            }),
            "invalid is_connected_to_cloud_services_or_client",
        ),
        (
            json!({
                "is_security_or_optimization_software": "ok",
            }),
            "invalid is_security_or_optimization_software",
        ),
        (
            json!({
                "is_supported_by_current_os": "don't know",
            }),
            "invalid is_supported_by_current_os",
        ),
        (
            json!({
                "review_notes": "A".repeat(1001),
            }),
            "review notes too long",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let update_software_review_response = server
            .patch_request(
                &format!("{}/{}", software_review_url, software_review_id),
                Some(invalid_body.to_string()),
                Some(&token.unwrap()),
            )
            .await?;
        (assert_eq!(
            400,
            update_software_review_response.status().as_u16(),
            "API did not fail with a 400 status when the payload was {}",
            error_message,
        ),);
    }

    Ok(())
}

#[tokio::test]
async fn update_software_review_with_missing_fields_rejected() -> Result<()> {
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

    let update_software_review_body = json!({});

    let update_software_review_response = server
        .patch_request(
            &format!("{}/{}", software_review_url, software_review_id),
            Some(update_software_review_body.to_string()),
            Some(&token.unwrap()),
        )
        .await?;
    assert_eq!(400, update_software_review_response.status().as_u16());

    Ok(())
}
