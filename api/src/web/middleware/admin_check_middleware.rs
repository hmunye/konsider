use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::Response;
use axum::RequestExt;
use sqlx::PgPool;
use uuid::Uuid;

use crate::model::TypedSession;
use crate::server::AppState;
use crate::{Error, Result, UserRole};

// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(name = "checking user role", skip(state, request, next))]
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
// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(name = "getting user role", skip(user_id, db_pool))]
async fn get_user_role(user_id: Uuid, db_pool: &PgPool) -> Result<UserRole> {
    let row = sqlx::query!(
        r#"
        SELECT role AS "role: UserRole"
        FROM "user"
        WHERE id = $1
        "#,
        user_id,
    )
    .fetch_one(db_pool)
    .await
    .map_err(|err| {
        Error::UnexpectedError(std::sync::Arc::new(err), "Failed to get user role".into())
    })?;

    Ok(row.role as UserRole)
}
