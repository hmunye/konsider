use axum::extract::{self, State};
use axum::http::StatusCode;
use secrecy::ExposeSecret;
use serde::Deserialize;

use crate::idempotency::{get_key_status, save_key_status, IdempotencyKey, IdempotencyStatus};
use crate::model::TypedSession;
use crate::server::AppState;
use crate::web::users::compute_password_hash;
use crate::web::users::insert_user;
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
    skip(state, session, payload),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_create_user(
    State(state): State<AppState>,
    session: TypedSession,
    extract::Json(payload): extract::Json<CreatePayload>,
) -> Result<StatusCode> {
    let idempotency_key: IdempotencyKey = payload
        .idempotency_key
        .try_into()
        .map_err(|_| Error::IdempotencyKeyError)?;

    if let Some(current_user_id) = session.get_user_id().await? {
        tracing::Span::current().record(
            "request_initiator",
            tracing::field::display(&current_user_id),
        );

        let key_status =
            get_key_status(&state.redis_pool, &idempotency_key, current_user_id).await?;

        match key_status {
            // Request has already been processed, return early
            IdempotencyStatus::Processed => return Ok(StatusCode::NO_CONTENT),
            // New request made, handle normally
            IdempotencyStatus::NotProcessed => {
                // Validate request payload
                payload.user.parse()?;

                let password_hash = compute_password_hash(payload.user.password.expose_secret())?;

                insert_user(&state, &payload.user, password_hash).await?;

                // Save idempotency key so duplicate requests are not processed
                save_key_status(&state.redis_pool, &idempotency_key, current_user_id).await?;
            }
        }
    }

    Ok(StatusCode::CREATED)
}
