use reqwest::header;
use serde_json::json;

use crate::common::{spawn_server, Result};

#[tokio::test]
async fn get_all_users_successful() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);

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

    let test_cases = vec![
        format!("{}/api/v1/users?page=2&per_page=4", server.addr),
        format!("{}/api/v1/users?page=2", server.addr),
        format!("{}/api/v1/users?per_page=4", server.addr),
        format!("{}/api/v1/users?sort=-name", server.addr),
        format!("{}/api/v1/users?sort=name", server.addr),
        format!("{}/api/v1/users?sort=-email", server.addr),
        format!("{}/api/v1/users?sort=email", server.addr),
        format!("{}/api/v1/users?sort=-role", server.addr),
        format!("{}/api/v1/users?sort=role", server.addr),
        format!("{}/api/v1/users?filter=role:ADMIN", server.addr),
        format!("{}/api/v1/users?filter=name:Amy", server.addr),
        format!(
            "{}/api/v1/users?filter=email:admin@brockport.edu",
            server.addr
        ),
    ];

    for vaild_url in test_cases {
        let get_user_response = server
            .get_request(&vaild_url, Some(&token.unwrap()))
            .await?;
        assert_eq!(200, get_user_response.status().as_u16());
    }

    Ok(())
}

#[tokio::test]
async fn get_all_users_with_invalid_query_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);

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

    let test_cases = vec![
        (
            format!("{}/api/v1/users?page=ac", server.addr),
            "invalid query param for page",
        ),
        (
            format!("{}/api/v1/users?per_page=ac", server.addr),
            "invalid query param for per_page",
        ),
        (
            format!("{}/api/v1/users?sort=33", server.addr),
            "invalid query param for sort",
        ),
        (
            format!("{}/api/v1/users?sort=password_hash", server.addr),
            "invalid query param for sort",
        ),
        (
            format!("{}/api/v1/users?filter=password_hash:$argon", server.addr),
            "invalid query param for filter",
        ),
        (
            format!("{}/api/v1/users?f=22", server.addr),
            "query param does not exist",
        ),
    ];

    for (invaild_url, error_message) in test_cases {
        let get_user_response = server
            .get_request(&invaild_url, Some(&token.unwrap()))
            .await?;
        (assert_eq!(
            400,
            get_user_response.status().as_u16(),
            "API did not fail with a 400 status when the payload was {}",
            error_message,
        ),);
    }

    Ok(())
}

#[tokio::test]
async fn get_user_using_invalid_role_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);
    let users_url = format!("{}/api/v1/users", server.addr);

    // Uses 'Reviewer' test user credentials
    let login_body = json!({
        "email": server.test_users[0].email,
        "password": server.test_users[0].password
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

    let get_user_response = server
        .get_request(&users_url, Some(&token.unwrap()))
        .await?;
    assert_eq!(403, get_user_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn sql_injection_get_users_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);

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

    let test_cases = vec![
        (
            format!("{}/api/v1/users?filter=role:ADMIN' OR 1=1 --", server.addr),
            "SQL injection attempt on role filter with OR 1=1",
        ),
        (
            format!(
                "{}/api/v1/users?filter=role:ADMIN' UNION SELECT NULL, NULL, NULL --",
                server.addr
            ),
            "SQL injection attempt with UNION SELECT",
        ),
        (
            format!(
                "{}/api/v1/users?filter=email:admin@example.com' OR 1=1 --",
                server.addr
            ),
            "SQL injection attempt on email filter with OR 1=1",
        ),
        (
            format!(
                "{}/api/v1/users?filter=role:ADMIN; DROP TABLE user_account;",
                server.addr
            ),
            "SQL injection attempt with DROP TABLE statement in role filter",
        ),
        (
            format!(
                "{}/api/v1/users?filter=role:ADMIN' OR EXISTS(SELECT * FROM user_account WHERE 1=1) --",
                server.addr
            ),
            "SQL injection attempt with nested EXISTS query",
        ),
        (
            format!("{}/api/v1/users?filter=role:ADMIN' --", server.addr),
            "SQL injection attempt with single-line comment in role filter",
        ),
        (
            format!(
                "{}/api/v1/users?filter=email:admin@example.com' /* comment */",
                server.addr
            ),
            "SQL injection attempt with multi-line comment in email filter",
        ),
        (
            format!(
                "{}/api/v1/users?filter=role:ADMIN' OR pg_sleep(5) --",
                server.addr
            ),
            "SQL injection attempt with time delay function in role filter",
        ),
        (
            format!("{}/api/v1/users?filter=role:ADMIN' AND 1=0 --", server.addr),
            "SQL injection attempt with AND 1=0 to bypass filtering",
        ),
        (
            format!(
                "{}/api/v1/users?filter=email:admin@example.com' AND 1=1",
                server.addr
            ),
            "SQL injection attempt with AND 1=1 on email filter",
        ),
        (
            format!("{}/api/v1/users?filter=role:ADMIN' OR 'a'='a", server.addr),
            "SQL injection attempt with OR 'a'='a' in role filter",
        ),
        (
            format!("{}/api/v1/users?filter=role:ADMIN' LIMIT 1 --", server.addr),
            "SQL injection attempt with LIMIT clause in role filter",
        ),
        (
            format!(
                "{}/api/v1/users?filter=email:admin@example.com' ORDER BY 1 --",
                server.addr
            ),
            "SQL injection attempt with ORDER BY clause in email filter",
        ),
    ];

    for (invaild_url, error_message) in test_cases {
        let get_user_response = server
            .get_request(&invaild_url, Some(&token.unwrap()))
            .await?;
        (assert_eq!(
            400,
            get_user_response.status().as_u16(),
            "API did not fail with a 400 status when the payload was {}",
            error_message,
        ),);
    }

    Ok(())
}
