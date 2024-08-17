use axum::extract::{self, State};
use axum::http::StatusCode;
use axum::response::Response;
use serde::Deserialize;
use serde_json::json;

use crate::web::server::AppState;
use crate::{Error, Result};

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    email: String,
    password: String,
}

#[tracing::instrument(
    name = "User Login",
    skip(state, payload),
    fields(
        user_email = %payload.email,
    )
)]
pub async fn login(
    State(state): State<AppState>,
    extract::Json(payload): extract::Json<LoginPayload>,
) -> Result<Response<String>> {
    let _user_id = match sqlx::query!(
        r#"
        SELECT id, password_hash
        FROM "user"
        WHERE email = $1
        "#,
        payload.email,
    )
    .fetch_one(&state.db_pool)
    .await
    {
        Ok(row) => {
            if verify_password(&payload.password, &row.password_hash) {
                row.id
            } else {
                return Err(Error::LoginFail);
            }
        }
        Err(err) => return Err(Error::DatabaseError(err.to_string())),
    };

    // TODO: Set Cookie using User ID

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

fn verify_password(provided_password: &str, stored_hash: &str) -> bool {
    // TODO: Implement password verification
    provided_password == stored_hash
}
