use reqwest::header;
use serde_json::json;

use crate::common::{spawn_server, Result};

#[tokio::test]
async fn get_all_software_successful() -> Result<()> {
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
    assert_eq!(200, login_response.status().as_u16());

    let token = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok());
    assert!(token.is_some(), "JWT should be present");

    let test_cases = vec![
        format!("{}/api/v1/software?page=2&per_page=4", server.addr),
        format!("{}/api/v1/software?page=2", server.addr),
        format!("{}/api/v1/software?per_page=4", server.addr),
        format!("{}/api/v1/software?sort=software_name", server.addr),
        format!("{}/api/v1/software?sort=developer_name", server.addr),
        format!("{}/api/v1/software?sort=-software_name", server.addr),
        format!("{}/api/v1/software?sort=-developer_name", server.addr),
        format!("{}/api/v1/software?filter=software_name:Xcode", server.addr),
        format!(
            "{}/api/v1/software?filter=developer_name:Apple",
            server.addr
        ),
    ];

    for valid_url in test_cases {
        let get_software_response = server
            .get_request(&valid_url, Some(&token.unwrap()))
            .await?;
        (
            assert_eq!(200, get_software_response.status().as_u16()),
            "API did not succeed with a 200 status when the url was {}",
            valid_url,
        );
    }

    Ok(())
}

#[tokio::test]
async fn get_all_software_with_invalid_query_rejected() -> Result<()> {
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
    assert_eq!(200, login_response.status().as_u16());

    let token = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok());
    assert!(token.is_some(), "JWT should be present");

    let test_cases = vec![
        (
            format!("{}/api/v1/software?page=ac", server.addr),
            "invalid query param for page",
        ),
        (
            format!("{}/api/v1/software?per_page=ac", server.addr),
            "invalid query param for per_page",
        ),
        (
            format!("{}/api/v1/software?sort=33", server.addr),
            "invalid query param for sort",
        ),
        (
            format!("{}/api/v1/software?sort=created_at", server.addr),
            "invalid query param for sort",
        ),
        (
            format!("{}/api/v1/software?filter=created_at:NOW()", server.addr),
            "invalid query param for filter",
        ),
        (
            format!("{}/api/v1/software?f=22", server.addr),
            "query param does not exist",
        ),
    ];

    for (invaild_url, error_message) in test_cases {
        let get_software_response = server
            .get_request(&invaild_url, Some(&token.unwrap()))
            .await?;
        (assert_eq!(
            400,
            get_software_response.status().as_u16(),
            "API did not fail with a 400 status when the payload was {}",
            error_message,
        ),);
    }

    Ok(())
}

#[tokio::test]
async fn get_software_using_missing_token_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let software_url = format!("{}/api/v1/software", server.addr);

    let get_software_response = server.get_request(&software_url, None).await?;
    assert_eq!(401, get_software_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn sql_injection_get_software_rejected() -> Result<()> {
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
    assert_eq!(200, login_response.status().as_u16());

    let token = login_response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok());
    assert!(token.is_some(), "JWT should be present");

    let test_cases = vec![
        (
            format!("{}/api/v1/software?filter=software_name:IT' OR 1=1 --", server.addr),
            "SQL injection attempt on role filter with OR 1=1",
        ),
        (
            format!(
                "{}/api/v1/software?filter=software_name:IT' UNION SELECT NULL, NULL, NULL --",
                server.addr
            ),
            "SQL injection attempt with UNION SELECT",
        ),
        (
            format!(
                "{}/api/v1/software?filter=developer_name:admin@example.com' OR 1=1 --",
                server.addr
            ),
            "SQL injection attempt on developer_name filter with OR 1=1",
        ),
        (
            format!(
                "{}/api/v1/software?filter=software_name:IT; DROP TABLE user_account;",
                server.addr
            ),
            "SQL injection attempt with DROP TABLE statement in role filter",
        ),
        (
            format!(
                "{}/api/v1/software?filter=software_name:IT' OR EXISTS(SELECT * FROM user_account WHERE 1=1) --",
                server.addr
            ),
            "SQL injection attempt with nested EXISTS query",
        ),
        (
            format!("{}/api/v1/software?filter=software_name:IT' --", server.addr),
            "SQL injection attempt with single-line comment in role filter",
        ),
        (
            format!(
                "{}/api/v1/software?filter=developer_name:admin@example.com' /* comment */",
                server.addr
            ),
            "SQL injection attempt with multi-line comment in developer_name filter",
        ),
        (
            format!(
                "{}/api/v1/software?filter=software_name:IT' OR pg_sleep(5) --",
                server.addr
            ),
            "SQL injection attempt with time delay function in role filter",
        ),
        (
            format!("{}/api/v1/software?filter=software_name:IT' AND 1=0 --", server.addr),
            "SQL injection attempt with AND 1=0 to bypass filtering",
        ),
        (
            format!(
                "{}/api/v1/software?filter=developer_name:admin@example.com' AND 1=1",
                server.addr
            ),
            "SQL injection attempt with AND 1=1 on developer_name filter",
        ),
        (
            format!("{}/api/v1/software?filter=software_name:IT' OR 'a'='a", server.addr),
            "SQL injection attempt with OR 'a'='a' in role filter",
        ),
        (
            format!("{}/api/v1/software?filter=software_name:IT' LIMIT 1 --", server.addr),
            "SQL injection attempt with LIMIT clause in role filter",
        ),
        (
            format!(
                "{}/api/v1/software?filter=developer_name:admin@example.com' ORDER BY 1 --",
                server.addr
            ),
            "SQL injection attempt with ORDER BY clause in developer_name filter",
        ),
    ];

    for (invaild_url, error_message) in test_cases {
        let get_software_response = server
            .get_request(&invaild_url, Some(&token.unwrap()))
            .await?;
        (assert_eq!(
            400,
            get_software_response.status().as_u16(),
            "API did not fail with a 400 status when the payload was {}",
            error_message,
        ),);
    }

    Ok(())
}
