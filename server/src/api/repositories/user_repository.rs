use secrecy::{ExposeSecret, SecretString};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{Error, Result};

#[tracing::instrument(
    name = "updating user password in database",
    skip(user_id, password_hash, db_pool)
)]
pub async fn update_user_password(
    user_id: Uuid,
    password_hash: SecretString,
    db_pool: &PgPool,
) -> Result<()> {
    match sqlx::query!(
        r#"
        UPDATE user_account 
        SET password_hash = $1
        WHERE id = $2
        RETURNING id
        "#,
        password_hash.expose_secret(),
        user_id
    )
    .fetch_optional(db_pool)
    .await
    {
        Ok(Some(_)) => Ok(()),
        Ok(None) => Err(Error::PgNotFoundError),
        Err(err) => Err(Error::from(err)),
    }
}
