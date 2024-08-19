mod common;

use api::UserRole;
use common::spawn_server;

use serde_json::json;

#[tokio::test]
async fn login_returns_200_status() {
    let client = reqwest::Client::new();
    let server = spawn_server().await;
    let url = format!("{}/auth/login", server.addr);

    let user_role = UserRole::Reviewer;

    // Create user to test against
    let _ = sqlx::query!(
        r#"
        INSERT INTO "user" (name, email, password_hash, role)
        VALUES ($1, $2, $3, $4)
        "#,
        "John",
        "john@gmail.com",
        "password123",
        user_role as UserRole,
    )
    .execute(&server.db_pool)
    .await
    .unwrap();

    // Payload
    let body = json!({
        "email": "john@gmail.com",
        "password": "password123"
    });

    // Request
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
    assert_eq!(Some(16), response.content_length());
}

#[tokio::test]
async fn login_returns_500_status() {
    let client = reqwest::Client::new();
    let server = spawn_server().await;
    let url = format!("{}/auth/login", server.addr);

    // Payload (User should not exist in db)
    let body = json!({
        "email": "test",
        "password": "test"
    });

    // Request
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(500, response.status().as_u16());
}
