use secrecy::SecretString;
use sqlx::PgPool;

use crate::api::models::UserRole;
use crate::{Error, Result};

#[tracing::instrument(
    name = "fetching user credentials by email from database",
    skip(email, db_pool)
)]
pub async fn fetch_credentials_by_email<'a>(
    email: &'a str,
    db_pool: &PgPool,
) -> Result<Option<(uuid::Uuid, SecretString, UserRole)>> {
    let row = sqlx::query!(
        r#"
        SELECT id, password_hash, role AS "role: UserRole"
        FROM user_account
        WHERE email = $1
        "#,
        email,
    )
    .fetch_optional(db_pool)
    .await
    .map_err(Error::from)?
    .map(|row| {
        (
            row.id,
            SecretString::new(row.password_hash.into()),
            // Should be safe to unwrap, since role will always have a value
            // Will return `None` if invalid email is used
            row.role.unwrap(),
        )
    });

    Ok(row)
}

#[tracing::instrument(
    name = "fetching user credentials by id from database",
    skip(user_id, db_pool)
)]
pub async fn fetch_credentials_by_user_id(
    user_id: uuid::Uuid,
    db_pool: &PgPool,
) -> Result<Option<(SecretString, UserRole)>> {
    let row = sqlx::query!(
        r#"
        SELECT password_hash, role AS "role: UserRole"
        FROM user_account
        WHERE id = $1
        "#,
        user_id,
    )
    .fetch_optional(db_pool)
    .await
    .map_err(Error::from)?
    .map(|row| {
        (
            SecretString::new(row.password_hash.into()),
            // Should be safe to unwrap, since role will always have a value
            // Will return `None` if invalid email is used
            row.role.unwrap(),
        )
    });

    Ok(row)
}
