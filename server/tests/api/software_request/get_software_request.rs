use reqwest::header;
use serde_json::json;

use crate::common::{spawn_server, Result};

#[tokio::test]
async fn get_all_software_requests_successful() -> Result<()> {
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
        format!("{}/api/v1/requests?page=2&per_page=4", server.addr),
        format!("{}/api/v1/requests?page=2", server.addr),
        format!("{}/api/v1/requests?per_page=4", server.addr),
        format!(
            "{}/api/v1/requests?filter=td_request_id:36472091",
            server.addr
        ),
        format!("{}/api/v1/requests?filter=software_name:Zoom", server.addr),
        format!(
            "{}/api/v1/requests?filter=requester_email:john@example.com",
            server.addr
        ),
    ];

    for valid_url in test_cases {
        let get_software_requests_response = server
            .get_request(&valid_url, Some(&token.unwrap()))
            .await?;
        (
            assert_eq!(200, get_software_requests_response.status().as_u16()),
            "API did not succeed with a 200 status when the url was {}",
            valid_url,
        );
    }

    Ok(())
}

#[tokio::test]
async fn get_all_software_requests_with_invalid_query_rejected() -> Result<()> {
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
            format!("{}/api/v1/requests?page=ac", server.addr),
            "invalid query param for page",
        ),
        (
            format!("{}/api/v1/requests?per_page=ac", server.addr),
            "invalid query param for per_page",
        ),
        (
            format!("{}/api/v1/requests?filter=created_at:NOW()", server.addr),
            "invalid query param for filter",
        ),
        (
            format!("{}/api/v1/requests?sort=22", server.addr),
            "sort query param is not allowed",
        ),
    ];

    for (invalid_url, error_message) in test_cases {
        let get_software_requests_response = server
            .get_request(&invalid_url, Some(&token.unwrap()))
            .await?;
        assert_eq!(
            400,
            get_software_requests_response.status().as_u16(),
            "API did not fail with a 400 status when the payload was {}",
            error_message,
        );
    }

    Ok(())
}

#[tokio::test]
async fn get_software_requests_using_missing_token_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let requests_url = format!("{}/api/v1/requests", server.addr);

    let get_software_requests_response = server.get_request(&requests_url, None).await?;
    assert_eq!(401, get_software_requests_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn sql_injection_get_software_requests_rejected() -> Result<()> {
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
            format!("{}/api/v1/requests?filter=td_request_id:36472091' OR 1=1 --", server.addr),
            "SQL injection attempt on role filter with OR 1=1",
        ),
        (
            format!(
                "{}/api/v1/requests?filter=td_request_id:36472091' UNION SELECT NULL, NULL, NULL --",
                server.addr
            ),
            "SQL injection attempt with UNION SELECT",
        ),
        (
            format!(
                "{}/api/v1/requests?filter=td_request_id:36472091' OR 1=1 --",
                server.addr
            ),
            "SQL injection attempt on developer_name filter with OR 1=1",
        ),
        (
            format!(
                "{}/api/v1/requests?filter=td_request_id:36472091; DROP TABLE user_account;",
                server.addr
            ),
            "SQL injection attempt with DROP TABLE statement in role filter",
        ),
        (
            format!(
                "{}/api/v1/requests?filter=td_request_id:36472091' OR EXISTS(SELECT * FROM user_account WHERE 1=1) --",
                server.addr
            ),
            "SQL injection attempt with nested EXISTS query",
        ),
        (
            format!("{}/api/v1/requests?td_request_id:36472091' --", server.addr),
            "SQL injection attempt with single-line comment in role filter",
        ),
        (
            format!(
                "{}/api/v1/requests?filter=td_request_id:36472091' /* comment */",
                server.addr
            ),
            "SQL injection attempt with multi-line comment in developer_name filter",
        ),
        (
            format!(
                "{}/api/v1/requests?filter=td_request_id:36472091' OR pg_sleep(5) --",
                server.addr
            ),
            "SQL injection attempt with time delay function in role filter",
        ),
        (
            format!("{}/api/v1/requests?filter=td_request_id:36472091' AND 1=0 --", server.addr),
            "SQL injection attempt with AND 1=0 to bypass filtering",
        ),
        (
            format!(
                "{}/api/v1/requests?filter=td_request_id:36472091' AND 1=1",
                server.addr
            ),
            "SQL injection attempt with AND 1=1 on developer_name filter",
        ),
        (
            format!("{}/api/v1/requests?filter=td_request_id:36472091' OR 'a'='a", server.addr),
            "SQL injection attempt with OR 'a'='a' in role filter",
        ),
        (
            format!("{}/api/v1/requests?filter=td_request_id:36472091' LIMIT 1 --", server.addr),
            "SQL injection attempt with LIMIT clause in role filter",
        ),
        (
            format!(
                "{}/api/v1/requests?filter=td_request_id:36472091' ORDER BY 1 --",
                server.addr
            ),
            "SQL injection attempt with ORDER BY clause in developer_name filter",
        ),
    ];

    for (invaild_url, error_message) in test_cases {
        let get_software_requests_response = server
            .get_request(&invaild_url, Some(&token.unwrap()))
            .await?;
        (assert_eq!(
            400,
            get_software_requests_response.status().as_u16(),
            "API did not fail with a 400 status when the payload was {}",
            error_message,
        ),);
    }

    Ok(())
}
