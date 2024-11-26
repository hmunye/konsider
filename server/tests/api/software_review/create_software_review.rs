use reqwest::header;
use serde_json::json;
use uuid::Uuid;

use crate::common::{spawn_server, Result};

#[tokio::test]
async fn create_software_review_successful() -> Result<()> {
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

    Ok(())
}

#[tokio::test]
async fn create_software_review_duplicate_rejected() -> Result<()> {
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

    let dup_create_software_review_response = server
        .post_request(
            &software_review_url,
            Some(create_software_review_body.to_string()),
            Some(&token.unwrap()),
        )
        .await?;
    assert_eq!(409, dup_create_software_review_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn create_software_request_using_missing_token_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let software_review_url = format!("{}/api/v1/reviews", server.addr);

    let create_software_review_body = json!({
        "software_request": {
            "td_request_id": "12345678",
            "software": {
                "id": Uuid::new_v4(),
                "software_name": "Test Software",
                "software_version": "1.0.0",
                "developer_name": "Test Developer",
                "description": "A test software application",
            },
            "requester": {
                "id": Uuid::new_v4(),
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
            None,
        )
        .await?;
    assert_eq!(401, create_software_review_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn create_software_review_with_invalid_fields_rejected() -> Result<()> {
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
        .and_then(|value| value.to_str().ok())
        .expect("JWT should be present");

    let test_cases = vec![
        (
            json!({
                "software_request": {
                    "td_request_id": "",
                    "software": {
                        "id": Uuid::new_v4(),
                        "software_name": "Test Software",
                        "software_version": "1.0.0",
                        "developer_name": "Test Developer",
                        "description": "A test software application"
                    },
                    "requester": {
                        "id": Uuid::new_v4(),
                        "name": "John",
                        "email": "john@gmail.com",
                        "department": "IT"
                    }
                },
                "reviewer_id": server.test_users[1].id,
                "is_supported": "TRUE",
                "is_current_version": "INVALID_VALUE",
                "is_reputation_good": "TRUE",
                "is_installation_from_developer": "TRUE",
                "is_local_admin_required": "FALSE",
                "is_connected_to_brockport_cloud": "FALSE",
                "is_connected_to_cloud_services_or_client": "FALSE",
                "is_security_or_optimization_software": "FALSE",
                "is_supported_by_current_os": "TRUE",
                "review_notes": "All conditions satisfied."
            }),
            "Invalid value for `is_current_version` field",
        ),
        (
            json!({
                "software_request": {
                    "td_request_id": "12345678",
                    "software": {
                        "id": Uuid::new_v4(),
                        "software_name": "",
                        "software_version": "1.0.0",
                        "developer_name": "Test Developer",
                        "description": "A test software application"
                    },
                    "requester": {
                        "id": Uuid::new_v4(),
                        "name": "John",
                        "email": "john@gmail.com",
                        "department": "IT"
                    }
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
            }),
            "Empty `software_name` field",
        ),
        (
            json!({
                "software_request": {
                    "td_request_id": "12345678",
                    "software": {
                        "id": Uuid::new_v4(),
                        "software_name": "Test Software",
                        "software_version": "1.0.0",
                        "developer_name": "Test Developer",
                        "description": "A test software application"
                    },
                    "requester": {
                        "id": Uuid::new_v4(),
                        "name": "John",
                        "email": "invalid-email",
                        "department": "IT"
                    }
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
            }),
            "Invalid email in `requester` field",
        ),
        (
            json!({
                "software_request": {
                    "td_request_id": "12345678",
                    "software": {
                        "id": Uuid::new_v4(),
                        "software_name": "Test Software",
                        "software_version": "1.0.0",
                        "developer_name": "Test Developer",
                        "description": "A test software application"
                    },
                    "requester": {
                        "id": Uuid::new_v4(),
                        "name": "John",
                        "email": "john@gmail.com",
                        "department": "IT"
                    }
                },
                "reviewer_id": server.test_users[1].id,
                "is_supported": "INVALID_VALUE",
                "is_current_version": "TRUE",
                "is_reputation_good": "TRUE",
                "is_installation_from_developer": "TRUE",
                "is_local_admin_required": "FALSE",
                "is_connected_to_brockport_cloud": "FALSE",
                "is_connected_to_cloud_services_or_client": "FALSE",
                "is_security_or_optimization_software": "FALSE",
                "is_supported_by_current_os": "TRUE",
                "review_notes": "All conditions satisfied."
            }),
            "Invalid value for `is_supported` field",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let create_software_review_response = server
            .post_request(
                &software_review_url,
                Some(invalid_body.to_string()),
                Some(&token),
            )
            .await?;
        assert_eq!(
            400,
            create_software_review_response.status().as_u16(),
            "API did not fail with a 400 status when the payload was {}",
            error_message,
        );
    }

    Ok(())
}

#[tokio::test]
async fn create_software_with_missing_fields_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);
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
        .and_then(|value| value.to_str().ok())
        .expect("JWT should be present");

    let test_cases = vec![
        (
            json!({
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
            }),
            "missing `software_request`",
        ),
        (
            json!({
                "software_request": {
                    "software": {
                        "id": Uuid::new_v4(),
                        "software_name": "Test Software",
                        "software_version": "1.0.0",
                        "developer_name": "Test Developer",
                        "description": "A test software application"
                    },
                    "requester": {
                        "id": Uuid::new_v4(),
                        "name": "John",
                        "email": "john@gmail.com",
                        "department": "IT"
                    }
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
            }),
            "missing `td_request_id`",
        ),
        (
            json!({
                "software_request": {
                    "td_request_id": "12345678",
                    "requester": {
                        "id": Uuid::new_v4(),
                        "name": "John",
                        "email": "john@gmail.com",
                        "department": "IT"
                    }
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
            }),
            "missing `software` section",
        ),
        (
            json!({
                "software_request": {
                    "td_request_id": "12345678",
                    "software": {
                        "id": Uuid::new_v4(),
                        "software_name": "Test Software",
                        "software_version": "1.0.0",
                        "developer_name": "Test Developer",
                        "description": "A test software application"
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
            }),
            "missing `requester` section",
        ),
        (
            json!({
                "software_request": {
                    "td_request_id": "12345678",
                    "software": {
                        "id": Uuid::new_v4(),
                        "software_version": "1.0.0",
                        "developer_name": "Test Developer",
                        "description": "A test software application"
                    },
                    "requester": {
                        "id": Uuid::new_v4(),
                        "name": "John",
                        "email": "john@gmail.com",
                        "department": "IT"
                    }
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
            }),
            "missing `software_name` in `software`",
        ),
        (
            json!({
                "software_request": {
                    "td_request_id": "12345678",
                    "software": {
                        "id": Uuid::new_v4(),
                        "software_name": "Test Software",
                        "software_version": "1.0.0",
                        "developer_name": "Test Developer",
                        "description": "A test software application"
                    },
                    "requester": {
                        "id": Uuid::new_v4(),
                        "email": "john@gmail.com",
                        "department": "IT"
                    }
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
            }),
            "missing `name` in `requester`",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let create_software_request_response = server
            .post_request(
                &software_request_url,
                Some(invalid_body.to_string()),
                Some(&token),
            )
            .await?;
        assert_eq!(
            400,
            create_software_request_response.status().as_u16(),
            "API did not fail with a 400 status when the payload was {}",
            error_message,
        );
    }

    Ok(())
}
