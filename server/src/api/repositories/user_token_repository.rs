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
        ON CONFLICT (user_id) DO UPDATE
        SET jti = $1, revoked = FALSE
        RETURNING jti
        "#,
        jti,
        user_id
    )
    .fetch_optional(db_pool)
    .await
    {
        Ok(Some(_)) => Ok(()),
        Ok(None) => Err(Error::PgNotFoundError),
        Err(err) => match err.as_database_error().and_then(|db_err| db_err.code()) {
            Some(code) if code == "23503" => Err(Error::PgKeyViolation),
            _ => Err(Error::from(err)),
        },
    }
}

#[tracing::instrument(name = "fetching valid user tokens from database", skip(db_pool))]
pub async fn fetch_valid_tokens(db_pool: &PgPool) -> Result<Vec<(Uuid, Uuid)>> {
    let rows = sqlx::query!(
        r#"
        SELECT jti, user_id
        FROM user_token
        WHERE revoked = FALSE
        "#
    )
    .fetch_all(db_pool)
    .await
    .map_err(Error::from)?;

    let valid_tokens: Vec<(Uuid, Uuid)> =
        rows.into_iter().map(|row| (row.jti, row.user_id)).collect();

    Ok(valid_tokens)
}

#[tracing::instrument(name = "updating user token in database", skip(user_id, db_pool))]
pub async fn update_user_token(user_id: Uuid, db_pool: &PgPool) -> Result<Uuid> {
    match sqlx::query!(
        r#"
        UPDATE user_token
        SET revoked = TRUE
        WHERE user_id = $1
        RETURNING jti
        "#,
        user_id
    )
    .fetch_optional(db_pool)
    .await
    {
        Ok(Some(row)) => Ok(row.jti),
        Ok(None) => Err(Error::PgNotFoundError),
        Err(err) => match err.as_database_error().and_then(|db_err| db_err.code()) {
            Some(code) if code == "23503" => Err(Error::PgKeyViolation),
            _ => Err(Error::from(err)),
        },
    }
}
