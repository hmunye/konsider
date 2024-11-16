use sqlx::PgPool;
use uuid::Uuid;

use crate::{Error, Result};

#[tracing::instrument(
    name = "inserting new user token in database",
    skip(jti, user_id, db_pool)
)]
pub async fn insert_user_token(jti: Uuid, user_id: &Uuid, db_pool: &PgPool) -> Result<()> {
    match sqlx::query!(
        r#"
        INSERT INTO user_token (jti, user_id)
        VALUES ($1, $2)
        "#,
        jti,
        user_id
    )
    .execute(db_pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => match err.as_database_error().and_then(|db_err| db_err.code()) {
            Some(code) if code == "23503" => Err(Error::PgKeyViolation),
            Some(code) if code == "23505" => Err(Error::PgRecordExists),
            _ => Err(Error::from(err)),
        },
    }
}

#[tracing::instrument(name = "fetching revoked tokens from database", skip(db_pool))]
pub async fn fetch_revoked_tokens(db_pool: &PgPool) -> Result<Vec<Uuid>> {
    let rows = sqlx::query!(
        r#"
        SELECT jti
        FROM user_token
        WHERE revoked = TRUE
        "#
    )
    .fetch_all(db_pool)
    .await
    .map_err(Error::from)?;

    let revoked_tokens: Vec<Uuid> = rows.into_iter().map(|row| row.jti).collect();

    Ok(revoked_tokens)
}
