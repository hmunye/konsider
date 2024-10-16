use axum::extract::{self, Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use uuid::Uuid;

use crate::idempotency::{get_key_status, save_key_status, IdempotencyKey, IdempotencyStatus};
use crate::model::TypedSession;
use crate::server::AppState;
use crate::web::users::delete_user;
use crate::{Error, Result};

// ---------------------------------------------------------------------------------------------------------------
#[derive(Debug, Deserialize)]
pub struct DeletePayload {
    pub idempotency_key: String,
}
// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(
    name = "deleting user", 
    // Any values in 'skip' won't be included in logs
    skip(state, session, payload),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_delete_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    session: TypedSession,
    extract::Json(payload): Json<DeletePayload>,
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
            IdempotencyStatus::Processed => return Ok(StatusCode::IM_A_TEAPOT),
            // New request made, handle normally
            IdempotencyStatus::NotProcessed => {
                delete_user(&state, &user_id).await?;

                // Save idempotency key so duplicate requests are not processed
                save_key_status(&state.redis_pool, &idempotency_key, current_user_id).await?;
            }
        }
    }

    Ok(StatusCode::NO_CONTENT)
}
