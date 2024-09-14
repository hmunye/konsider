use axum::extract::{self, State};
use axum::http::StatusCode;
use secrecy::ExposeSecret;
use serde::Deserialize;

use crate::idempotency::{get_key_status, save_key_status, IdempotencyKey, IdempotencyStatus};
use crate::model::TypedSession;
use crate::server::AppState;
use crate::web::admin::compute_password_hash;
use crate::web::admin::insert_user;
use crate::{Error, Result, User};

// ---------------------------------------------------------------------------------------------------------------
#[derive(Debug, Deserialize)]
pub struct CreatePayload {
    pub user: User,
    pub idempotency_key: String,
}
// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(
    name = "creating user", 
    // Any values in 'skip' won't be included in logs
    skip(state, payload),
    fields(
        user_email = tracing::field::Empty,
    )
)]
pub async fn api_create_user(
    State(state): State<AppState>,
    session: TypedSession,
    extract::Json(payload): extract::Json<CreatePayload>,
) -> Result<StatusCode> {
    tracing::Span::current().record("user_email", tracing::field::display(&payload.user.email));

    // Validate request payload
    payload.user.parse()?;

    let idempotency_key: IdempotencyKey = payload
        .idempotency_key
        .try_into()
        .map_err(|_| Error::IdempotencyKeyError)?;

    if let Some(user_id) = session.get_user_id().await? {
        let key_status = get_key_status(&state.redis_pool, &idempotency_key, user_id).await?;

        match key_status {
            IdempotencyStatus::Processed => return Ok(StatusCode::OK),
            IdempotencyStatus::NotProcessed => {
                let password_hash = compute_password_hash(payload.user.password.expose_secret())?;

                insert_user(&state, &payload.user, password_hash).await?;

                save_key_status(&state.redis_pool, &idempotency_key, user_id).await?;
            }
        }
    }

    Ok(StatusCode::CREATED)
}
