// TODO: Eventually turn this into middleware for all admin endpoints

use sqlx::PgPool;
use uuid::Uuid;

use crate::model::TypedSession;
use crate::server::AppState;
use crate::{ServerError, UserRole};

// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(name = "checking user role", skip(state, session))]
pub async fn check_if_admin(state: &AppState, session: &TypedSession) -> Result<bool, ServerError> {
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

    // Check if user has admin role
    match user_role {
        UserRole::Admin => Ok(true),
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
