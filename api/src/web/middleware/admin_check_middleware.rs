use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::Response;
use axum::RequestExt;

use crate::model::TypedSession;
use crate::server::AppState;
use crate::web::auth::get_user_role;
use crate::{Error, Result, UserRole};

// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(name = "checking for admin role", skip(state, request, next))]
pub async fn reject_non_admin_users(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response> {
    // Get session from incoming request
    let session = request.extract_parts::<TypedSession>().await?;

    let user_role = if let Some(user_id) = session.get_user_id().await? {
        get_user_role(user_id, &state.db_pool).await?
    } else {
        return Err(Error::NoAuthProvidedError)?;
    };

    // If user_role is `Admin` then continue with the request
    match user_role {
        UserRole::Admin => {
            let response = next.run(request).await;
            Ok(response)
        }
        _ => Err(Error::InvalidRoleError)?,
    }
}
