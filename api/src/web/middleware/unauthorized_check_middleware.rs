use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::RequestExt;

use crate::model::TypedSession;
use crate::{Error, Result};

// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(name = "checking for existing session", skip(request, next))]
pub async fn reject_unauthorized_users(mut request: Request, next: Next) -> Result<Response> {
    // Get session from incoming request
    let session = request.extract_parts::<TypedSession>().await?;

    // Check for a user id associated with the session
    match session.get_user_id().await? {
        Some(_) => {
            let response = next.run(request).await;
            Ok(response)
        }
        None => Err(Error::NoAuthProvidedError)?,
    }
}
