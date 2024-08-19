use axum::extract::{self, State};
use axum::http::StatusCode;
use axum::response::Response;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::web::server::AppState;
use crate::{Error, Result};

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    email: String,
    password: String,
}

#[derive(Debug)]
pub struct UserDetails {
    user_id: Uuid,
    password_hash: String,
}

#[tracing::instrument(
    name = "User login attempt", 
    // Won't include in logs
    skip(state, payload),
    fields(
        user_email = %payload.email
    )
)]
pub async fn api_login(
    State(state): State<AppState>,
    extract::Json(payload): extract::Json<LoginPayload>,
) -> Result<Response<String>> {
    let user_details = login_user(&state, &payload).await?;

    verify_password(&payload.password, &user_details.password_hash)?;

    // TODO: Set Cookie using User ID
    let _user_id = user_details.user_id;

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

#[tracing::instrument(name = "Logging in user", skip(state, payload))]
async fn login_user(state: &AppState, payload: &LoginPayload) -> Result<UserDetails> {
    let (user_id, password_hash) = match sqlx::query!(
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
        Ok(row) => (row.id, row.password_hash),
        Err(err) => {
            tracing::error!("Failed to execute query: {:?}", err);
            return Err(Error::DatabaseError(err.to_string()));
        }
    };

    Ok(UserDetails {
        user_id,
        password_hash,
    })
}

fn verify_password(provided_password: &str, stored_hash: &str) -> Result<()> {
    // TODO: Implement password verification
    if provided_password == stored_hash {
        Ok(())
    } else {
        Err(Error::LoginFail)
    }
}
