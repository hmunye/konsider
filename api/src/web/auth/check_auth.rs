use axum::http::StatusCode;

use crate::Result;

// ---------------------------------------------------------------------------------------------------------------
pub async fn api_check_auth() -> Result<StatusCode> {
    // This endpoint is behind the `reject_unauthorized_users` middleware, checking for user
    // sessions. Allows for the client to check for unauthorized users
    Ok(StatusCode::NO_CONTENT)
}
