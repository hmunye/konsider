use sqlx::PgPool;
use uuid::Uuid;

use crate::api::repositories::{fetch_valid_tokens, insert_user_token, update_user_token};
use crate::Result;

#[tracing::instrument(name = "saving user token", skip(jti, user_id, db_pool))]
pub async fn save_user_token(jti: Uuid, user_id: &Uuid, db_pool: &PgPool) -> Result<()> {
    insert_user_token(jti, user_id, db_pool).await
}

#[tracing::instrument(name = "getting all valid tokens", skip(db_pool))]
pub async fn get_valid_tokens(db_pool: &PgPool) -> Result<Vec<(Uuid, Uuid)>> {
    fetch_valid_tokens(db_pool).await
}

#[tracing::instrument(name = "revoking user token", skip(user_id, db_pool))]
pub async fn revoke_user_token(user_id: Uuid, db_pool: &PgPool) -> Result<Uuid> {
    update_user_token(user_id, db_pool).await
}
