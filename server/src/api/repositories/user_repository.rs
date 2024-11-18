use secrecy::{ExposeSecret, SecretString};
use sqlx::PgPool;
use uuid::Uuid;

use crate::api::models::{UserDTO, UserRole};
use crate::api::utils::Metadata;
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

#[derive(Debug, sqlx::FromRow)]
struct UserRecordCount {
    count: i64,
    id: Uuid,
    name: String,
    email: String,
    role: UserRole,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[tracing::instrument(
    name = "fetching all users from database",
    skip(
        sort_column,
        sort_direction,
        page,
        per_page,
        filter_field,
        filter_value,
        db_pool
    )
)]
pub async fn fetch_all_users(
    sort_column: String,
    sort_direction: String,
    page: usize,
    per_page: usize,
    filter_field: Option<String>,
    filter_value: Option<String>,
    db_pool: &PgPool,
) -> Result<(Vec<UserDTO>, Metadata)> {
    let limit = per_page;
    let offset = (page - 1) * per_page;

    let query = if let (Some(field), Some(_)) = (filter_field.as_ref(), filter_value.as_ref()) {
        format!(
            r#"
            SELECT count(*) OVER(), id, name, email, role, created_at
            FROM user_account
            WHERE (to_tsvector('simple', {}::TEXT) @@ plainto_tsquery('simple', $1))
            ORDER BY {} {}, id ASC
            LIMIT {} OFFSET {}
            "#,
            field, sort_column, sort_direction, limit, offset
        )
    } else {
        format!(
            r#"
            SELECT count(*) OVER(), id, name, email, role, created_at
            FROM user_account
            ORDER BY {} {}, id ASC
            LIMIT {} OFFSET {}
            "#,
            sort_column, sort_direction, limit, offset
        )
    };

    let query = sqlx::query_as::<_, UserRecordCount>(&query);

    // Bind the user-supplied value only if it exists
    let query = if let Some(value) = filter_value {
        query.bind(value)
    } else {
        query
    };

    let records = query.fetch_all(db_pool).await.map_err(Error::from)?;

    let total_records = records.first().map_or(0, |record| record.count);

    let user_records: Vec<UserDTO> = records
        .into_iter()
        .map(|record| UserDTO {
            id: Some(record.id),
            name: record.name,
            email: record.email,
            role: record.role,
            created_at: Some(record.created_at),
        })
        .collect();

    let metadata = Metadata::calculate_metadata(total_records, page, per_page);

    Ok((user_records, metadata))
}
