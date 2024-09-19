// TODO: Update tests to compare response to an existing record

use reqwest::header;
use serde_json::json;
use uuid::Uuid;

use crate::common::spawn_server;

// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn get_all_users_successful() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);

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
        format!("{}/v1/users?page=2&per_page=4", server.addr),
        format!("{}/v1/users?page=2", server.addr),
        format!("{}/v1/users?per_page=4", server.addr),
        format!("{}/v1/users?sort=-name", server.addr),
        format!("{}/v1/users?sort=name", server.addr),
        format!("{}/v1/users?sort=-email", server.addr),
        format!("{}/v1/users?sort=email", server.addr),
        format!("{}/v1/users?sort=-role", server.addr),
        format!("{}/v1/users?sort=role", server.addr),
    ];

    for vaild_url in test_cases {
        let update_user_response = server
            .get_request(&vaild_url, Some(&session_id.unwrap()), None)
            .await;
        assert_eq!(200, update_user_response.status().as_u16());
    }
}
// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn get_all_users_with_invalid_query_rejected() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);

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
        (
            format!("{}/v1/users?page=ac", server.addr),
            "invalid query param for page",
        ),
        (
            format!("{}/v1/users?per_page=ac", server.addr),
            "invalid query param for per_page",
        ),
        (
            format!("{}/v1/users?sort=33", server.addr),
            "invalid query param for sort",
        ),
        (
            format!("{}/v1/users?sort=password_hash", server.addr),
            "invalid query param for sort",
        ),
        (
            format!("{}/v1/users?f=22", server.addr),
            "query param does not exist",
        ),
    ];

    for (invaild_url, error_message) in test_cases {
        let update_user_response = server
            .get_request(&invaild_url, Some(&session_id.unwrap()), None)
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
async fn get_user_by_id_successful() {
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

    let get_user_response = server
        .get_request(&users_url, Some(&session_id.unwrap()), None)
        .await;
    assert_eq!(200, get_user_response.status().as_u16());
}
// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn get_user_using_invalid_role_rejected() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);
    let users_url = format!("{}/v1/users", server.addr);

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

    let get_user_response = server
        .get_request(&users_url, Some(&session_id.unwrap()), None)
        .await;
    assert_eq!(403, get_user_response.status().as_u16());
}
// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn get_user_invaild_id_rejected() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);

    let test_user_id = Uuid::new_v4().to_string();
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

    let get_user_response = server
        .get_request(&users_url, Some(&session_id.unwrap()), None)
        .await;
    assert_eq!(404, get_user_response.status().as_u16());
}
