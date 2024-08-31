use axum::extract::{self, State};
use axum::http::StatusCode;

use crate::model::TypedSession;
use crate::server::AppState;
use crate::web::auth::{validate_credentials, Credentials};
use crate::ServerError;

// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(
    name = "user login attempt", 
    // Any values in 'skip' won't be included in logs
    skip(state, session, payload),
    fields(
        user_email = %payload.email
    )
)]
pub async fn api_login(
    State(state): State<AppState>,
    session: TypedSession,
    extract::Json(payload): extract::Json<Credentials>,
) -> Result<StatusCode, ServerError> {
    match validate_credentials(&state, payload).await {
        Ok(user_id) => {
            // Rotating session id prevents session fixation attacks
            session.cycle().await?;

            // Create session with user id
            session.insert_user_id(user_id).await?;
        }
        Err(err) => return Err(ServerError::LoginError(err.to_string()))?,
    };

    Ok(StatusCode::OK)
}
