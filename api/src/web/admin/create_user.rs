use axum::extract::{self, State};
use axum::http::StatusCode;
use secrecy::ExposeSecret;

use crate::server::AppState;
use crate::web::admin::compute_password_hash;
use crate::web::admin::insert_user;
use crate::{Result, User};

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
    extract::Json(payload): extract::Json<User>,
) -> Result<StatusCode> {
    tracing::Span::current().record("user_email", tracing::field::display(&payload.email));

    // Validate request payload
    payload.parse()?;

    let password_hash = compute_password_hash(payload.password.expose_secret())?;

    insert_user(&state, &payload, password_hash).await?;

    Ok(StatusCode::OK)
}
