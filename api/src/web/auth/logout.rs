use axum::http::StatusCode;

use crate::model::TypedSession;
use crate::ServerError;

// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(
    name = "user logout", 
    // Any values in 'skip' won't be included in logs
    skip(session),
)]
pub async fn api_logout(session: TypedSession) -> Result<StatusCode, ServerError> {
    if session
        .get_user_id()
        .await
        .map_err(|err| ServerError::UnexpectedError(err.to_string()))?
        .is_none()
    {
        return Err(ServerError::NoAuthProvided)?;
    } else {
        match session.log_out_user().await {
            Ok(..) => (),
            Err(err) => return Err(ServerError::LogoutError(err.to_string()))?,
        };
    };

    Ok(StatusCode::OK)
}
