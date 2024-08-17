use axum::extract::{self, State};
use axum::http::StatusCode;
use axum::response::Response;
use serde_json::json;
use uuid::Uuid;

use crate::web::server::AppState;
use crate::{Error, Result, User, UserRole};

#[tracing::instrument(
    name = "Creating New User",
    skip(state, payload),
    fields(
        user_name = %payload.name,
        user_email = %payload.email,
    )
)]
pub async fn create_user(
    State(state): State<AppState>,
    extract::Json(payload): extract::Json<User>,
) -> Result<Response<String>> {
    match payload.parse() {
        Ok(payload) => payload,
        Err(_) => return Err(Error::CreateUserFail),
    };

    // TODO: Hash and salt password
    let password_hash = &payload.password;

    let request_id = Uuid::new_v4();

    tracing::info!(
        "request-id {} - Adding '{}' '{}' as a new user",
        request_id,
        payload.email,
        payload.name
    );

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
