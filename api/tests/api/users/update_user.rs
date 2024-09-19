use reqwest::header;
use serde_json::json;
use uuid::Uuid;

use crate::common::spawn_server;
use api::UserRole;

// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn update_user_successful() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);

    // Uses 'Reviewer' test user id
    let test_user_id = server.test_users[0].id;
    let users_url = format!("{}/v1/users/{}", server.addr, test_user_id);

    // Store test user details in Vec before update for later comparison
    let original_test_user = vec![
        server.test_users[0].name.clone(),
        server.test_users[0].email.clone(),
        server.test_users[0].password.clone(),
        server.test_users[0].role.to_string(),
    ];

    // Uses 'Admin' test user credentials
    let body = json!({
        "email": server.test_users[1].email,
        "password": server.test_users[1].password
    });

    let login_response = server
        .post_request(&login_url, Some(body.to_string()), None, None)
        .await;
    assert_eq!(204, login_response.status().as_u16());

    let session_id = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|str| str.split(";").nth(0));
    assert!(session_id.is_some(), "Session ID should be present");

    let test_cases = vec![
        json!({
            "user": {
                "name": "John",
                "email": "test@gmail.com",
                "password": "testing123132312",
                "role": "Reviewer",
            },
            "idempotency_key": Uuid::new_v4().to_string(),
        }),
        json!({
            "user": {
                "name": "Smith",
            },
            "idempotency_key": Uuid::new_v4().to_string(),
        }),
        json!({
            "user": {
                "email": "newtest@gmail.com",
            },
            "idempotency_key": Uuid::new_v4().to_string(),
        }),
        json!({
            "user": {
                "password": "kjsadkjbsahkdbas",
            },
            "idempotency_key": Uuid::new_v4().to_string(),
        }),
        json!({
            "user": {
                "role": "Admin",
            },
            "idempotency_key": Uuid::new_v4().to_string(),
        }),
    ];

    for valid_body in test_cases {
        let update_user_response = server
            .patch_request(
                &users_url,
                Some(valid_body.to_string()),
                Some(&session_id.unwrap()),
                None,
            )
            .await;

        assert_eq!(204, update_user_response.status().as_u16());
    }

    let row = sqlx::query!(
        r#"
        SELECT name, email, password_hash, role AS "role: UserRole" 
        FROM users
        WHERE id = $1
        "#,
        test_user_id
    )
    .fetch_one(&server.db_pool)
    .await
    .map(|row| vec![row.name, row.email, row.password_hash, row.role.to_string()])
    .unwrap();

    // TODO: Updated to compare each field
    assert!(!(original_test_user == row))
}
// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn update_user_using_invalid_role_rejected() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);

    // Uses 'Reviewer' test user id
    let test_user_id = server.test_users[0].id;
    let users_url = format!("{}/v1/users/{}", server.addr, test_user_id);

    // Uses 'Reviewer' test user credentials
    let body = json!({
        "email": server.test_users[0].email,
        "password": server.test_users[0].password
    });

    let login_response = server
        .post_request(&login_url, Some(body.to_string()), None, None)
        .await;
    assert_eq!(204, login_response.status().as_u16());

    let session_id = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|str| str.split(";").nth(0));
    assert!(session_id.is_some(), "Session ID should be present");

    let body = json!({
        "user": {
            "name": "John",
            "email": "test@sdsd.com",
            "password": "password123",
            "role": "Reviewer"
        },
        "idempotency_key": Uuid::new_v4().to_string()
    });

    let update_user_response = server
        .patch_request(
            &users_url,
            Some(body.to_string()),
            Some(&session_id.unwrap()),
            None,
        )
        .await;
    assert_eq!(403, update_user_response.status().as_u16());
}
// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn update_user_with_invalid_id_rejected() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);

    let test_user_id = Uuid::new_v4();
    let users_url = format!("{}/v1/users/{}", server.addr, test_user_id);

    // Uses 'Admin' test user credentials
    let body = json!({
        "email": server.test_users[1].email,
        "password": server.test_users[1].password
    });

    let login_response = server
        .post_request(&login_url, Some(body.to_string()), None, None)
        .await;
    assert_eq!(204, login_response.status().as_u16());

    let session_id = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|str| str.split(";").nth(0));
    assert!(session_id.is_some(), "Session ID should be present");

    let body = json!({
        "user": {
            "name": "John",
            "email": "test@sdsd.com",
            "password": "password123",
            "role": "Reviewer"
        },
        "idempotency_key": Uuid::new_v4().to_string()
    });

    let update_user_response = server
        .patch_request(
            &users_url,
            Some(body.to_string()),
            Some(&session_id.unwrap()),
            None,
        )
        .await;
    // Returns a 404 status code to indicate the user does not exist
    assert_eq!(404, update_user_response.status().as_u16());
}
// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn update_user_with_invalid_fields_rejected() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);

    // Uses 'Reviewer' test user id
    let test_user_id = server.test_users[0].id;
    let users_url = format!("{}/v1/users/{}", server.addr, test_user_id);

    // Uses 'Admin' test user credentials
    let body = json!({
        "email": server.test_users[1].email,
        "password": server.test_users[1].password
    });

    let login_response = server
        .post_request(&login_url, Some(body.to_string()), None, None)
        .await;
    assert_eq!(204, login_response.status().as_u16());

    let session_id = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|str| str.split(";").nth(0));
    assert!(session_id.is_some(), "Session ID should be present");

    let idempotency_key = Uuid::new_v4().to_string();

    let test_cases = vec![
        (
            json!({
                "user": {
                    "name": "John)asdd$",
                },
                "idempotency_key": idempotency_key.clone(),
            }),
            "invalid name",
        ),
        (
            json!({
                "user": {
                    "email": "newtestgmail.com",
                },
                "idempotency_key": idempotency_key.clone(),
            }),
            "invalid email",
        ),
        (
            json!({
                "user": {
                    "password": "kj",
                },
                "idempotency_key": idempotency_key.clone(),
            }),
            "invalid password",
        ),
        (
            json!({
                "user": {
                    "role": "ad",
                },
                "idempotency_key": idempotency_key.clone(),
            }),
            "invalid role",
        ),
    ];
    for (invalid_body, error_message) in test_cases {
        let update_user_response = server
            .patch_request(
                &users_url,
                Some(invalid_body.to_string()),
                Some(&session_id.unwrap()),
                None,
            )
            .await;

        (
            assert_eq!(400, update_user_response.status().as_u16()),
            "API did not fail with a 400 status when the payload was {}",
            error_message,
        );
    }
}
// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn update_user_with_missing_fields_rejected() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);

    // Uses 'Reviewer' test user id
    let test_user_id = server.test_users[0].id;
    let users_url = format!("{}/v1/users/{}", server.addr, test_user_id);

    // Uses 'Admin' test user credentials
    let body = json!({
        "email": server.test_users[1].email,
        "password": server.test_users[1].password
    });

    let login_response = server
        .post_request(&login_url, Some(body.to_string()), None, None)
        .await;
    assert_eq!(204, login_response.status().as_u16());

    let session_id = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|str| str.split(";").nth(0));
    assert!(session_id.is_some(), "Session ID should be present");

    let body = json!({});

    let update_user_response = server
        .patch_request(
            &users_url,
            Some(body.to_string()),
            Some(&session_id.unwrap()),
            None,
        )
        .await;
    assert_eq!(400, update_user_response.status().as_u16());
}
// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn update_user_is_idempotent() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);

    // Uses 'Reviewer' test user id
    let test_user_id = server.test_users[0].id;
    let users_url = format!("{}/v1/users/{}", server.addr, test_user_id);

    // Uses 'Admin' test user credentials
    let body = json!({
        "email": server.test_users[1].email,
        "password": server.test_users[1].password
    });

    let login_response = server
        .post_request(&login_url, Some(body.to_string()), None, None)
        .await;
    assert_eq!(204, login_response.status().as_u16());

    let session_id = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|str| str.split(";").nth(0));
    assert!(session_id.is_some(), "Session ID should be present");

    let body = json!({
        "user": {
            "password": "kjsadkjbsahkdbas",
        },
        "idempotency_key": Uuid::new_v4().to_string(),
    });

    let update_user_response = server
        .patch_request(
            &users_url,
            Some(body.to_string()),
            Some(&session_id.unwrap()),
            None,
        )
        .await;
    assert_eq!(204, update_user_response.status().as_u16());

    let dup_update_user_response = server
        .patch_request(
            &users_url,
            Some(body.to_string()),
            Some(&session_id.unwrap()),
            None,
        )
        .await;
    assert_eq!(418, dup_update_user_response.status().as_u16());
}
// ---------------------------------------------------------------------------------------------------------------
// TODO: This test is flaky, fix it
#[tokio::test]
async fn update_user_optimistic_concurrency_control() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);

    // Uses 'Reviewer' test user id
    let test_user_id = server.test_users[0].id;
    let users_url = format!("{}/v1/users/{}", server.addr, test_user_id);

    // Uses first 'Admin' test user credentials
    let body = json!({
        "email": server.test_users[1].email,
        "password": server.test_users[1].password
    });

    let login_response_1 = server
        .post_request(&login_url, Some(body.to_string()), None, None)
        .await;
    assert_eq!(204, login_response_1.status().as_u16());

    let session_id_1 = login_response_1
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|str| str.split(";").nth(0));
    assert!(session_id_1.is_some(), "Session ID should be present");

    // Uses second 'Admin' test user credentials
    let body = json!({
        "email": server.test_users[3].email,
        "password": server.test_users[3].password
    });

    let login_response_2 = server
        .post_request(&login_url, Some(body.to_string()), None, None)
        .await;
    assert_eq!(204, login_response_2.status().as_u16());

    let session_id_2 = login_response_2
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|str| str.split(";").nth(0));
    assert!(session_id_2.is_some(), "Session ID should be present");

    // First `Admin` updates the user
    let initial_body = json!({
        "user": {
            "name": "newnameinitial",
        },
        "idempotency_key": Uuid::new_v4().to_string(),
    });

    let update_user_response_1 = server.patch_request(
        &users_url,
        Some(initial_body.to_string()),
        Some(&session_id_1.unwrap()),
        None,
    );

    // Second `Admin` tries to update the user concurrently
    let stale_body = json!({
        "user": {
            "name": "newnamestale",
        },
        "idempotency_key": Uuid::new_v4().to_string(),
    });

    let update_user_response_2 = server.patch_request(
        &users_url,
        Some(stale_body.to_string()),
        Some(&session_id_2.unwrap()),
        None,
    );

    // Await both requests concurrently
    let (update_user_response_1, update_user_response_2) =
        tokio::join!(update_user_response_1, update_user_response_2);

    // The problem is that due to concurrency, it's unclear which request will succeed
    // and which will fail with a 409 Conflict
    let status_1 = update_user_response_1.status().as_u16();
    let status_2 = update_user_response_2.status().as_u16();

    // Determine which response is a conflict and which is successful
    let (conflict_response_status, success_response_status) = if status_1 == 409 {
        (status_1, status_2)
    } else {
        (status_2, status_1)
    };

    // Assert that one response is a conflict and the other is a success
    assert_eq!(
        409, conflict_response_status,
        "Expected conflict status for one update"
    );
    assert_eq!(
        204, success_response_status,
        "Expected success status for the other update"
    );
}
