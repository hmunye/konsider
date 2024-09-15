use axum::extract::{self, Path, State};
use axum::http::StatusCode;
use axum::Json;
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use uuid::Uuid;

use crate::idempotency::{get_key_status, save_key_status, IdempotencyKey, IdempotencyStatus};
use crate::model::TypedSession;
use crate::server::AppState;
use crate::web::admin::{compute_password_hash, fetch_user_by_id, update_user};
use crate::{Error, Result, UserRole};

// ---------------------------------------------------------------------------------------------------------------
// Errors if `role` is not either `Admin` or `Reviewer`
#[derive(Debug, Deserialize)]
pub struct UserOptional {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<Secret<String>>,
    pub role: Option<UserRole>,
}
// ---------------------------------------------------------------------------------------------------------------
#[derive(Debug, Deserialize)]
pub struct UpdatePayload {
    pub user: UserOptional,
    pub idempotency_key: String,
}
// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(
    name = "updating user details", 
    // Any values in 'skip' won't be included in logs
    skip(state, updating_user_id, session, payload),
    fields(
        updating_user_id = tracing::field::Empty,
    )
)]
pub async fn api_update_user(
    State(state): State<AppState>,
    Path(updating_user_id): Path<Uuid>,
    session: TypedSession,
    extract::Json(payload): Json<UpdatePayload>,
) -> Result<StatusCode> {
    tracing::Span::current().record(
        "updating_user_id",
        tracing::field::display(&updating_user_id),
    );

    let idempotency_key: IdempotencyKey = payload
        .idempotency_key
        .try_into()
        .map_err(|_| Error::IdempotencyKeyError)?;

    if let Some(current_user_id) = session.get_user_id().await? {
        let key_status =
            get_key_status(&state.redis_pool, &idempotency_key, current_user_id).await?;

        match key_status {
            // Request has already been processed, return early
            IdempotencyStatus::Processed => return Ok(StatusCode::IM_A_TEAPOT),
            // New request made, handle normally
            IdempotencyStatus::NotProcessed => {
                // Fetch user from database if a record exists
                let mut user = fetch_user_by_id(&state, &updating_user_id).await?;

                let mut changed_password = false;
                let mut fields_updated = false;

                // Apply any updates to the `User` struct locally
                if let Some(name) = &payload.user.name {
                    user.name = name.clone();
                    fields_updated = true;
                }

                if let Some(email) = &payload.user.email {
                    user.email = email.clone();
                    fields_updated = true;
                }

                if let Some(password) = &payload.user.password {
                    user.password = password.clone();
                    changed_password = true;
                    fields_updated = true;
                }

                if let Some(role) = &payload.user.role {
                    user.role = role.clone();
                    fields_updated = true;
                }

                // Return an error if no fields were updated
                if !fields_updated {
                    return Err(Error::NoUpdatesProvidedError);
                }

                // Validate the updated user, parsing depending on if the password was changed
                match changed_password {
                    true => {
                        user.parse()?;

                        if let Some(password) = &payload.user.password {
                            user.password = compute_password_hash(password.expose_secret())?.into();
                        }
                    }
                    false => user.parse_without_password()?,
                }

                update_user(&state, &user, &updating_user_id).await?;

                // Save idempotency key so duplicate requests are not processed
                save_key_status(&state.redis_pool, &idempotency_key, current_user_id).await?;
            }
        }
    }

    Ok(StatusCode::NO_CONTENT)
}
