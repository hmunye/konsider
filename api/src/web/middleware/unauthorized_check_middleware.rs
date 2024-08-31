use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::RequestExt;

use crate::model::TypedSession;
use crate::ServerError;

// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(name = "checking session state", skip(request, next))]
pub async fn reject_unauthorized_users(
    mut request: Request,
    next: Next,
) -> Result<Response, ServerError> {
    // Get session from incoming request
    let session = request
        .extract_parts::<TypedSession>()
        .await
        .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

    match session
        .get_user_id()
        .await
        .map_err(|err| ServerError::UnexpectedError(err.to_string()))?
    {
        Some(_) => {
            let response = next.run(request).await;
            Ok(response)
        }
        None => Err(ServerError::NoAuthProvided),
    }
}
