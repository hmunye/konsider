// TODO: Update tests to compare response to existing record

use reqwest::header;
use serde_json::json;

use crate::common::spawn_server;

// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn get_all_user_successful() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);
    let users_url = format!("{}/v1/admin/users", server.addr);

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
async fn get_user_by_id_successful() {
    let server = spawn_server().await;
    let login_url = format!("{}/v1/auth/login", server.addr);

    // Uses 'Reviewer' test user id
    let test_user_id = server.test_users[0].id;
    let users_url = format!("{}/v1/admin/users/{}", server.addr, test_user_id);

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
    let users_url = format!("{}/v1/admin/users", server.addr);

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
