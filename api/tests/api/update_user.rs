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
    let update_user_url = format!("{}/v1/admin/update-user/{}", server.addr, test_user_id);

    // Store test user details before update in Vec for later comparison
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
        .post_request(&login_url, Some(body.to_string()), None)
        .await;
    assert_eq!(200, login_response.status().as_u16());

    // TODO: Find out how to correctly preserve cookies without manual extraction
    let session_id = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|str| str.split(";").nth(0));

    let test_cases = vec![
        (
            json!({
                "name": "John",
                "email": "test@gmail.com",
                "password": "testing123132312",
                "role": "Reviewer",
            }),
            "updating all fields",
        ),
        (
            json!({
                "name": "Smith",
            }),
            "updating name",
        ),
        (
            json!({
                "email": "newtest@gmail.com",
            }),
            "updating email",
        ),
        (
            json!({
                "password": "kjsadkjbsahkdbas",
            }),
            "updating password",
        ),
        (
            json!({
                "role": "Admin",
            }),
            "updating role",
        ),
    ];

    for (valid_body, update_message) in test_cases {
        let update_user_response = server
            .patch_request(
                &update_user_url,
                Some(valid_body.to_string()),
                Some(&session_id.unwrap()),
            )
            .await;

        (
            assert_eq!(200, update_user_response.status().as_u16()),
            "API processed request with a 200 status when the payload was {}",
            update_message,
        );
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
    let update_user_url = format!("{}/v1/admin/update-user/{}", server.addr, test_user_id);

    // Uses 'Reviewer' test user credentials
    let body = json!({
        "email": server.test_users[0].email,
        "password": server.test_users[0].password
    });

    let login_response = server
        .post_request(&login_url, Some(body.to_string()), None)
        .await;
    assert_eq!(200, login_response.status().as_u16());

    // TODO: Find out how to correctly preserve cookies without manual extraction
    let session_id = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|str| str.split(";").nth(0));

    let body = json!({
        "name": "John",
        "email": "test@sdsd.com",
        "password": "password123",
        "role": "Reviewer"
    });

    let update_user_response = server
        .patch_request(
            &update_user_url,
            Some(body.to_string()),
            Some(&session_id.unwrap()),
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
    let update_user_url = format!("{}/v1/admin/update-user/{}", server.addr, test_user_id);

    // Uses 'Admin' test user credentials
    let body = json!({
        "email": server.test_users[1].email,
        "password": server.test_users[1].password
    });

    let login_response = server
        .post_request(&login_url, Some(body.to_string()), None)
        .await;
    assert_eq!(200, login_response.status().as_u16());

    // TODO: Find out how to correctly preserve cookies without manual extraction
    let session_id = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|str| str.split(";").nth(0));

    let body = json!({
        "name": "John",
        "email": "test@sdsd.com",
        "password": "password123",
        "role": "Reviewer"
    });

    let update_user_response = server
        .patch_request(
            &update_user_url,
            Some(body.to_string()),
            Some(&session_id.unwrap()),
        )
        .await;
    assert_eq!(404, update_user_response.status().as_u16());
}
// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn update_user_with_invalid_fields_rejected() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);

    // Uses 'Reviewer' test user id
    let test_user_id = server.test_users[0].id;
    let update_user_url = format!("{}/v1/admin/update-user/{}", server.addr, test_user_id);

    // Uses 'Admin' test user credentials
    let body = json!({
        "email": server.test_users[1].email,
        "password": server.test_users[1].password
    });

    let login_response = server
        .post_request(&login_url, Some(body.to_string()), None)
        .await;
    assert_eq!(200, login_response.status().as_u16());

    // TODO: Find out how to correctly preserve cookies without manual extraction
    let session_id = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|str| str.split(";").nth(0));

    let test_cases = vec![
        (
            json!({
                "name": "John)asdd$",
            }),
            "invalid name",
        ),
        (
            json!({
                "email": "newtestgmail.com",
            }),
            "invalid email",
        ),
        (
            json!({
                "password": "kj",
            }),
            "invalid password",
        ),
        (
            json!({
                "role": "ad",
            }),
            "invalid role",
        ),
    ];

    for (valid_body, update_message) in test_cases {
        let update_user_response = server
            .patch_request(
                &update_user_url,
                Some(valid_body.to_string()),
                Some(&session_id.unwrap()),
            )
            .await;

        (
            assert_eq!(400, update_user_response.status().as_u16()),
            "API processed with a 400 status when the payload was {}",
            update_message,
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
    let update_user_url = format!("{}/v1/admin/update-user/{}", server.addr, test_user_id);

    // Uses 'Admin' test user credentials
    let body = json!({
        "email": server.test_users[1].email,
        "password": server.test_users[1].password
    });

    let login_response = server
        .post_request(&login_url, Some(body.to_string()), None)
        .await;
    assert_eq!(200, login_response.status().as_u16());

    // TODO: Find out how to correctly preserve cookies without manual extraction
    let session_id = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|str| str.split(";").nth(0));

    let body = json!({});

    let update_user_response = server
        .patch_request(
            &update_user_url,
            Some(body.to_string()),
            Some(&session_id.unwrap()),
        )
        .await;
    assert_eq!(400, update_user_response.status().as_u16());
}
