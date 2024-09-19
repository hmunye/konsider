use reqwest::header;
use serde_json::json;
use uuid::Uuid;

use crate::common::spawn_server;

// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn delete_user_successful() {
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
        "idempotency_key": Uuid::new_v4().to_string()
    });

    let delete_user_response = server
        .delete_request(
            &users_url,
            Some(body.to_string()),
            Some(&session_id.unwrap()),
            None,
        )
        .await;
    assert_eq!(204, delete_user_response.status().as_u16());

    let row = sqlx::query!(
        r#"
        SELECT id
        FROM users
        WHERE id=$1 
        "#,
        test_user_id
    )
    .fetch_one(&server.db_pool)
    .await;

    assert!(!row.is_ok());
}
// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn delete_user_using_invalid_role_rejected() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);

    // Uses 'Reviewer' test user delete id
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
        "idempotency_key": Uuid::new_v4().to_string()
    });

    let delete_user_response = server
        .delete_request(
            &users_url,
            Some(body.to_string()),
            Some(&session_id.unwrap()),
            None,
        )
        .await;
    assert_eq!(403, delete_user_response.status().as_u16());
}
// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn delete_user_with_invalid_id_rejected() {
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
        "idempotency_key": Uuid::new_v4().to_string()
    });

    let delete_user_response = server
        .delete_request(
            &users_url,
            Some(body.to_string()),
            Some(&session_id.unwrap()),
            None,
        )
        .await;
    // Returns a 404 status code to indicate the user does not exist
    assert_eq!(404, delete_user_response.status().as_u16());
}
// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn delete_user_is_idempotent() {
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
        "idempotency_key": Uuid::new_v4().to_string()
    });

    let delete_user_response = server
        .delete_request(
            &users_url,
            Some(body.to_string()),
            Some(&session_id.unwrap()),
            None,
        )
        .await;
    assert_eq!(204, delete_user_response.status().as_u16());

    let dup_delete_user_response = server
        .delete_request(
            &users_url,
            Some(body.to_string()),
            Some(&session_id.unwrap()),
            None,
        )
        .await;
    assert_eq!(418, dup_delete_user_response.status().as_u16());
}
