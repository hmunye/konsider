use axum::extract::{self, State};
use axum::http::StatusCode;
use axum::response::Response;
use serde_json::json;

use crate::web::server::AppState;
use crate::{Error, Result, User, UserRole};

pub async fn create_user(
    State(state): State<AppState>,
    extract::Json(payload): extract::Json<User>,
) -> Result<Response<String>> {
    println!("->> {:<12} - create_user", "HANDLER");

    // Validate Payload
    match payload.validate_new_user() {
        Ok(payload) => payload,
        Err(_) => return Err(Error::CreateUserFail),
    };

    // TODO: Hash and salt password
    let password_hash = &payload.password;

    // Query to insert the user into the database
    match sqlx::query!(
        r#"
        INSERT INTO "user" (name, email, password_hash, role)
        VALUES ($1, $2, $3, $4)
        "#,
        payload.name,
        payload.email,
        password_hash,
        payload.role as UserRole,
    )
    .execute(&state.db_pool)
    .await
    {
        Ok(..) => (),
        Err(err) => return Err(Error::DatabaseError(err.to_string())),
    };

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
                "success": true,
            })
            .to_string(),
        )
        .unwrap();

    Ok(response)
}
