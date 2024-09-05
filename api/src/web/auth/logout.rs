use axum::http::StatusCode;

use crate::model::TypedSession;
use crate::{Error, Result};

// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(
    name = "logging out user", 
    // Any values in 'skip' won't be included in logs
    skip(session),
)]
pub async fn api_logout(session: TypedSession) -> Result<StatusCode> {
    // Make sure there is a valid session to be flushed
    if session.get_user_id().await?.is_none() {
        return Err(Error::NoAuthProvidedError)?;
    } else {
        session.log_out_user().await?;
    };

    Ok(StatusCode::OK)
}
