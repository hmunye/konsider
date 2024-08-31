use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::Response;
use axum::RequestExt;
use sqlx::PgPool;
use uuid::Uuid;

use crate::model::TypedSession;
use crate::server::AppState;
use crate::{ServerError, UserRole};

// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(name = "checking user role", skip(state, request, next))]
pub async fn reject_non_admin_users(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, ServerError> {
    // Get session from incoming request
    let session = request
        .extract_parts::<TypedSession>()
        .await
        .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

    let user_role = if let Some(user_id) = session
        .get_user_id()
        .await
        .map_err(|err| ServerError::UnexpectedError(err.to_string()))?
    {
        get_user_role(user_id, &state.db_pool)
            .await
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?
    } else {
        return Err(ServerError::NoAuthProvided);
    };

    // If user_role is `Admin` then continue with the request
    match user_role {
        UserRole::Admin => {
            let response = next.run(request).await;
            Ok(response)
        }
        _ => Err(ServerError::InvalidRole),
    }
}
// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(name = "getting user role", skip(user_id, db_pool))]
async fn get_user_role(user_id: Uuid, db_pool: &PgPool) -> Result<UserRole, ServerError> {
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
    .map_err(|err| ServerError::InsertUserError(err.to_string()))?;

    Ok(row.role as UserRole)
}
