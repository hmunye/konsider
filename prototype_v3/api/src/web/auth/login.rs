use axum::extract::{self, State};
use axum::http::StatusCode;
use serde::Deserialize;
use uuid::Uuid;

use crate::server::AppState;
use crate::{Error, Result};

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    email: String,
    password: String,
}

#[tracing::instrument(
    name = "user login attempt", 
    // Any values in 'skip' won't be included in logs
    skip(state, payload),
    fields(
        user_email = %payload.email
    )
)]
pub async fn api_login(
    State(state): State<AppState>,
    extract::Json(payload): extract::Json<LoginPayload>,
) -> Result<StatusCode> {
    let user_details = fetch_user(&state, &payload).await?;

    verify_password(&payload.password, &user_details.password_hash)?;

    // TODO: Set Cookie using User ID
    let _user_id = user_details.user_id;

    Ok(StatusCode::OK)
}

#[derive(Debug)]
pub struct UserDetails {
    user_id: Uuid,
    password_hash: String,
}

#[tracing::instrument(name = "fetching user details", skip(state, payload))]
async fn fetch_user(state: &AppState, payload: &LoginPayload) -> Result<UserDetails> {
    let result = sqlx::query!(
        r#"
        SELECT id, password_hash
        FROM "user"
        WHERE email = $1
        "#,
        payload.email,
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|err| Error::FetchUserError(err.to_string()))?;

    Ok(UserDetails {
        user_id: result.id,
        password_hash: result.password_hash,
    })
}

fn verify_password(provided_password: &str, stored_hash: &str) -> Result<()> {
    // TODO: Implement password verification
    if provided_password == stored_hash {
        Ok(())
    } else {
        Err(Error::LoginError(
            "password does not match stored hash".to_string(),
        ))
    }
}
