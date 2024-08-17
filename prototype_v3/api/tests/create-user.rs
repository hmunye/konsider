mod common;

use common::spawn_server;
use serde_json::json;

#[tokio::test]
async fn create_user_returns_200_status() {
    let client = reqwest::Client::new();
    let server = spawn_server().await;
    let url = format!("{}/admin/create-user", server.addr);

    let email: String = String::from("john@gmail.com");

    let body = json!({
        "name": "John",
        "email": email,
        "password": "password123",
        "role": "Reviewer"
    });

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await
        .expect("Failed to execute request.");

    let _ = sqlx::query!(
        r#"
        SELECT id
        FROM "user"
        WHERE email=$1 
        "#,
        email
    )
    .fetch_one(&server.db_pool)
    .await
    .unwrap();

    assert_eq!(200, response.status().as_u16());
    assert_eq!(Some(16), response.content_length());
}

#[tokio::test]
async fn create_user_returns_422_status() {
    let client = reqwest::Client::new();
    let server = spawn_server().await;
    let url = format!("{}/admin/create-user", server.addr);

    let test_cases = vec![
        (
            json!({
                "email": "test@gmail.com",
                "password": "testing123",
                "role": "Reviewer",
            }),
            "missing name.",
        ),
        (
            json!({
                "name": "John",
                "password": "testing123",
                "role": "Reviewer",
            }),
            "missing email.",
        ),
        (
            json!({
                "name": "John",
                "email": "test@gmail.com",
                "role": "Reviewer",
            }),
            "missing password.",
        ),
        (
            json!({
                "name": "John",
                "email": "test@gmail.com",
                "password": "testing123",
            }),
            "missing role.",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .body(invalid_body.to_string())
            .send()
            .await
            .expect("Failed to execute request.");

        (
            assert_eq!(422, response.status().as_u16()),
            "API did not fail with a 422 status when the payload was {}",
            error_message,
        );
    }
}

#[tokio::test]
async fn create_user_returns_500_status() {
    let client = reqwest::Client::new();
    let server = spawn_server().await;
    let url = format!("{}/admin/create-user", server.addr);

    let test_cases = vec![
        (
            json!({
                "name": "",
                "email": "test@gmail.com",
                "password": "testing123",
                "role": "Reviewer",
            }),
            "empty name.",
        ),
        (
            json!({
                "name": "John",
                "email": "",
                "password": "testing123",
                "role": "Reviewer",
            }),
            "empty email",
        ),
        (
            json!({
                "name": "John",
                "email": "test@gmail.com",
                "password": "",
                "role": "Reviewer",
            }),
            "empty password",
        ),
        // Would return a 422 status because role is from the UserRole enum
        //        (
        //            json!({
        //                "name": "John",
        //                "email": "test@gmail.com",
        //                "password": "testing123",
        //                "role": "",
        //            }),
        //            "empty role",
        //        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .body(invalid_body.to_string())
            .send()
            .await
            .expect("Failed to execute request.");

        (
            assert_eq!(500, response.status().as_u16()),
            "API did not fail with a 500 status when the payload was {}",
            error_message,
        );
    }
}
