use sqlx::PgPool;
use uuid::Uuid;

use crate::api::repositories::{fetch_valid_tokens, insert_user_token};
use crate::Result;

#[tracing::instrument(name = "save user token", skip(jti, user_id, db_pool))]
pub async fn save_user_token(jti: Uuid, user_id: &Uuid, db_pool: &PgPool) -> Result<()> {
    insert_user_token(jti, user_id, db_pool).await
}

#[tracing::instrument(name = "getting valid tokens", skip(db_pool))]
pub async fn get_valid_tokens(db_pool: &PgPool) -> Result<Vec<(Uuid, Uuid)>> {
    fetch_valid_tokens(db_pool).await
}
