use axum::extract::{self, State};
use axum::http::StatusCode;

use crate::model::TypedSession;
use crate::server::AppState;
use crate::web::auth::{validate_credentials, Credentials};
use crate::Result;

// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(
    name = "logging in user", 
    // Any values in 'skip' won't be included in logs
    skip(state, session, payload),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_login(
    State(state): State<AppState>,
    session: TypedSession,
    extract::Json(payload): extract::Json<Credentials>,
) -> Result<StatusCode> {
    let user_id = validate_credentials(&state, payload).await?;

    tracing::Span::current().record("request_initiator", tracing::field::display(&user_id));

    // Rotating session id prevents session fixation attacks
    session.cycle().await?;

    // Create session with user id
    session.insert_user_id(user_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
