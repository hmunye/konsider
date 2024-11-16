use secrecy::SecretString;
use sqlx::PgPool;

use crate::api::UserRole;
use crate::{Error, Result};

#[tracing::instrument(name = "fetching user credentials", skip(db_pool, email))]
pub async fn fetch_credentials<'a>(
    db_pool: &PgPool,
    email: &'a str,
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
            row.role.unwrap(),
        )
    });

    Ok(row)
}
