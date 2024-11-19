use reqwest::header;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::common::{spawn_server, Result};

#[tokio::test]
async fn update_user_successful() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);

    // Uses 'Reviewer' test user id
    let test_user_id = server.test_users[0].id;
    let users_url = format!("{}/api/v1/users/{}", server.addr, test_user_id);

    // Store test user details in Vec before update for later comparison
    let original_test_user = vec![
        server.test_users[0].name.clone(),
        server.test_users[0].email.clone(),
        server.test_users[0].role.to_string(),
    ];

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
        json!({
            "name": "John",
            "email": "test@gmail.com",
            "role": "REVIEWER",
        }),
        json!({
            "name": "Smith",
        }),
        json!({
            "email": "newtest@gmail.com",
        }),
        json!({
            "role": "ADMIN",
        }),
    ];

    for valid_body in test_cases {
        let update_user_response = server
            .patch_request(
                &users_url,
                Some(valid_body.to_string()),
                Some(&token.unwrap()),
            )
            .await?;
        (
            assert_eq!(204, update_user_response.status().as_u16()),
            "API did not succeed with a 204 status when the payload was {}",
            valid_body,
        );
    }

    let get_user_url = format!("{}/api/v1/users?filter=name:{}", server.addr, "Smith");

    let get_user_response = server
        .get_request(&get_user_url, Some(&token.unwrap()))
        .await?;
    assert_eq!(200, get_user_response.status().as_u16());

    let users = get_user_response.text().await.unwrap();

    let parsed_user: Value = serde_json::from_str(&users).unwrap();

    let user = &parsed_user["users"][0]["user"];

    let updated_user = vec![
        user["name"].as_str().unwrap_or_default().to_string(),
        user["email"].as_str().unwrap_or_default().to_string(),
        user["role"].as_str().unwrap_or_default().to_string(),
    ];

    assert!(!(original_test_user == updated_user));

    Ok(())
}

#[tokio::test]
async fn update_user_using_invalid_role_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);

    // Uses 'Reviewer' test user id
    let test_user_id = server.test_users[0].id;
    let users_url = format!("{}/api/v1/users/{}", server.addr, test_user_id);

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

    let update_user_body = json!({
        "name": "John",
        "email": "test@sdsd.com",
        "role": "REVIEWER"
    });

    let update_user_response = server
        .patch_request(
            &users_url,
            Some(update_user_body.to_string()),
            Some(&token.unwrap()),
        )
        .await?;
    assert_eq!(403, update_user_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn update_user_with_invalid_id_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);

    let test_user_id = Uuid::new_v4();
    let users_url = format!("{}/api/v1/users/{}", server.addr, test_user_id);

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

    let update_user_body = json!({
        "name": "John",
        "email": "test@sdsd.com",
        "role": "REVIEWER"
    });

    let update_user_response = server
        .patch_request(
            &users_url,
            Some(update_user_body.to_string()),
            Some(&token.unwrap()),
        )
        .await?;
    // Returns a 404 status code to indicate the user does not exist
    assert_eq!(404, update_user_response.status().as_u16());

    Ok(())
}

#[tokio::test]
async fn update_user_with_invalid_fields_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);

    // Uses 'Reviewer' test user id
    let test_user_id = server.test_users[0].id;
    let users_url = format!("{}/api/v1/users/{}", server.addr, test_user_id);

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
                "role": "ad",
            }),
            "invalid role",
        ),
    ];
    for (invalid_body, error_message) in test_cases {
        let update_user_response = server
            .patch_request(
                &users_url,
                Some(invalid_body.to_string()),
                Some(&token.unwrap()),
            )
            .await?;
        (assert_eq!(
            400,
            update_user_response.status().as_u16(),
            "API did not fail with a 400 status when the payload was {}",
            error_message,
        ),);
    }

    Ok(())
}

#[tokio::test]
async fn update_user_with_missing_fields_rejected() -> Result<()> {
    let server = spawn_server().await?;
    let login_url = format!("{}/api/v1/auth/login", server.addr);

    // Uses 'Reviewer' test user id
    let test_user_id = server.test_users[0].id;
    let users_url = format!("{}/api/v1/users/{}", server.addr, test_user_id);

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

    let update_user_body = json!({});

    let update_user_response = server
        .patch_request(
            &users_url,
            Some(update_user_body.to_string()),
            Some(&token.unwrap()),
        )
        .await?;
    assert_eq!(400, update_user_response.status().as_u16());

    Ok(())
}
