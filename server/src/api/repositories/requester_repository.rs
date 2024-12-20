use sqlx::PgPool;
use uuid::Uuid;

use crate::api::models::{Requester, RequesterDTO};
use crate::api::utils::Metadata;
use crate::{Error, Result};

#[derive(Debug, sqlx::FromRow)]
struct RequesterRecordCount {
    count: i64,
    id: Uuid,
    name: String,
    email: String,
    department: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[tracing::instrument(
    name = "fetching all requesters from database",
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
pub async fn fetch_all_requesters(
    sort_column: String,
    sort_direction: String,
    page: usize,
    per_page: usize,
    filter_field: Option<String>,
    filter_value: Option<String>,
    db_pool: &PgPool,
) -> Result<(Vec<RequesterDTO>, Metadata)> {
    let limit = per_page;
    let offset = (page - 1) * per_page;

    let query = if let (Some(field), Some(_)) = (filter_field.as_ref(), filter_value.as_ref()) {
        format!(
            r#"
            SELECT count(*) OVER(), id, name, email, department, created_at
            FROM requester
            WHERE (to_tsvector('simple', {}::TEXT) @@ plainto_tsquery('simple', $1))
            ORDER BY {} {}, id ASC
            LIMIT {} OFFSET {}
            "#,
            field, sort_column, sort_direction, limit, offset
        )
    } else {
        format!(
            r#"
            SELECT count(*) OVER(), id, name, email, department, created_at
            FROM requester
            ORDER BY {} {}, id ASC
            LIMIT {} OFFSET {}
            "#,
            sort_column, sort_direction, limit, offset
        )
    };

    let query = sqlx::query_as::<_, RequesterRecordCount>(&query);

    // Bind the user-supplied value only if it exists
    let query = if let Some(value) = filter_value {
        query.bind(value)
    } else {
        query
    };

    let records = query.fetch_all(db_pool).await.map_err(Error::from)?;

    let total_records = records.first().map_or(0, |record| record.count);

    let requester_records: Vec<RequesterDTO> = records
        .into_iter()
        .map(|record| RequesterDTO {
            id: Some(record.id),
            name: record.name,
            email: record.email,
            department: record.department,
            created_at: Some(record.created_at),
        })
        .collect();

    let metadata = Metadata::calculate_metadata(total_records, page, per_page);

    Ok((requester_records, metadata))
}

#[tracing::instrument(
    name = "fetching requester by id from database",
    skip(requester_id, db_pool)
)]
pub async fn fetch_requester_by_id(requester_id: Uuid, db_pool: &PgPool) -> Result<Requester> {
    let row = sqlx::query!(
        r#"
        SELECT id, name, email, department, created_at, updated_at, version
        FROM requester
        WHERE id = $1
        "#,
        requester_id
    )
    .fetch_optional(db_pool)
    .await
    .map_err(Error::from)?;

    match row {
        Some(row) => Ok(Requester {
            id: Some(row.id),
            name: row.name,
            email: row.email,
            department: row.department,
            created_at: row.created_at,
            updated_at: row.updated_at,
            version: row.version,
        }),
        None => Err(Error::PgNotFoundError),
    }
}

#[tracing::instrument(name = "inserting requester into database", skip(payload, db_pool))]
pub async fn insert_requester(payload: &Requester, db_pool: &PgPool) -> Result<Uuid> {
    match sqlx::query!(
        r#"
        INSERT INTO requester (name, email, department)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        payload.name,
        payload.email,
        payload.department,
    )
    .fetch_optional(db_pool)
    .await
    {
        Ok(Some(row)) => Ok(row.id),
        Ok(None) => Err(Error::PgNotFoundError),
        Err(err) => match err.as_database_error().and_then(|db_err| db_err.code()) {
            Some(code) if code == "23505" => Err(Error::PgRecordExists),
            _ => Err(Error::from(err)),
        },
    }
}

#[tracing::instrument(name = "deleting requester from database", skip(requester_id, db_pool))]
pub async fn delete_requester(requester_id: Uuid, db_pool: &PgPool) -> Result<()> {
    match sqlx::query!(
        r#"
        DELETE FROM requester
        WHERE id = $1
        RETURNING id
        "#,
        requester_id,
    )
    .fetch_optional(db_pool)
    .await
    {
        Ok(Some(_)) => Ok(()),
        Ok(None) => Err(Error::PgNotFoundError),
        Err(err) => match err.as_database_error().and_then(|db_err| db_err.code()) {
            Some(code) if code == "23503" => Err(Error::PgDependencyViolation),
            _ => Err(Error::from(err)),
        },
    }
}

#[tracing::instrument(
    name = "updating requester details in database",
    skip(requester, requester_id, db_pool)
)]
pub async fn update_requester(
    requester: Requester,
    requester_id: Uuid,
    db_pool: &PgPool,
) -> Result<()> {
    match sqlx::query!(
        r#"
        UPDATE requester
        SET name = $1, email = $2, department = $3, version = version + 1
        WHERE id = $4 AND version = $5
        RETURNING version
        "#,
        requester.name,
        requester.email,
        requester.department,
        requester_id,
        requester.version
    )
    .fetch_optional(db_pool)
    .await
    {
        Ok(Some(_)) => Ok(()),
        Ok(None) => Err(Error::PgNotFoundError),
        Err(err) => match err.as_database_error().and_then(|db_err| db_err.code()) {
            Some(code) if code == "23505" => Err(Error::PgRecordExists),
            _ => Err(Error::from(err)),
        },
    }
}
