use sqlx::PgPool;
use uuid::Uuid;

use crate::api::models::{Software, SoftwareDTO};
use crate::api::utils::Metadata;
use crate::{Error, Result};

#[derive(Debug, sqlx::FromRow)]
struct SoftwareRecordCount {
    count: i64,
    id: Uuid,
    software_name: String,
    software_version: String,
    developer_name: String,
    description: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[tracing::instrument(
    name = "fetching all software from database",
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
pub async fn fetch_all_software(
    sort_column: String,
    sort_direction: String,
    page: usize,
    per_page: usize,
    filter_field: Option<String>,
    filter_value: Option<String>,
    db_pool: &PgPool,
) -> Result<(Vec<SoftwareDTO>, Metadata)> {
    let limit = per_page;
    let offset = (page - 1) * per_page;

    let query = if let (Some(field), Some(_)) = (filter_field.as_ref(), filter_value.as_ref()) {
        format!(
            r#"
            SELECT count(*) OVER(), id, software_name, software_version, developer_name, description, created_at
            FROM software
            WHERE (to_tsvector('simple', {}::TEXT) @@ plainto_tsquery('simple', $1))
            ORDER BY {} {}, id ASC
            LIMIT {} OFFSET {}
            "#,
            field, sort_column, sort_direction, limit, offset
        )
    } else {
        format!(
            r#"
            SELECT count(*) OVER(), id, software_name, software_version, developer_name, description, created_at
            FROM software
            ORDER BY {} {}, id ASC
            LIMIT {} OFFSET {}
            "#,
            sort_column, sort_direction, limit, offset
        )
    };

    let query = sqlx::query_as::<_, SoftwareRecordCount>(&query);

    // Bind the user-supplied value only if it exists
    let query = if let Some(value) = filter_value {
        query.bind(value)
    } else {
        query
    };

    let records = query.fetch_all(db_pool).await.map_err(Error::from)?;

    let total_records = records.first().map_or(0, |record| record.count);

    let software_records: Vec<SoftwareDTO> = records
        .into_iter()
        .map(|record| SoftwareDTO {
            id: Some(record.id),
            software_name: record.software_name,
            software_version: record.software_version,
            developer_name: record.developer_name,
            description: record.description,
            created_at: Some(record.created_at),
        })
        .collect();

    let metadata = Metadata::calculate_metadata(total_records, page, per_page);

    Ok((software_records, metadata))
}

#[tracing::instrument(
    name = "fetching software by id from database",
    skip(software_id, db_pool)
)]
pub async fn fetch_software_by_id(software_id: Uuid, db_pool: &PgPool) -> Result<Software> {
    let row = sqlx::query!(
        r#"
        SELECT id, software_name, software_version, developer_name, description, created_at, updated_at, version
        FROM software
        WHERE id = $1
        "#,
        software_id
    )
    .fetch_optional(db_pool)
    .await
    .map_err(Error::from)?;

    match row {
        Some(row) => Ok(Software {
            id: Some(row.id),
            software_name: row.software_name,
            software_version: row.software_version,
            developer_name: row.developer_name,
            description: row.description,
            created_at: row.created_at,
            updated_at: row.updated_at,
            version: row.version,
        }),
        None => Err(Error::PgNotFoundError),
    }
}

#[tracing::instrument(name = "inserting software into database", skip(payload, db_pool))]
pub async fn insert_software(payload: &Software, db_pool: &PgPool) -> Result<()> {
    match sqlx::query!(
        r#"
        INSERT INTO software (software_name, software_version, developer_name, description)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
        payload.software_name,
        payload.software_version,
        payload.developer_name,
        payload.description,
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

#[tracing::instrument(name = "deleting software from database", skip(software_id, db_pool))]
pub async fn delete_software(software_id: Uuid, db_pool: &PgPool) -> Result<()> {
    match sqlx::query!(
        r#"
        DELETE FROM software
        WHERE id = $1
        RETURNING id
        "#,
        software_id,
    )
    .fetch_optional(db_pool)
    .await
    {
        Ok(Some(_)) => Ok(()),
        Ok(None) => Err(Error::PgNotFoundError),
        Err(err) => Err(Error::from(err)),
    }
}

#[tracing::instrument(
    name = "updating software details in database",
    skip(software, software_id, db_pool)
)]
pub async fn update_software(
    software: Software,
    software_id: Uuid,
    db_pool: &PgPool,
) -> Result<()> {
    match sqlx::query!(
        r#"
        UPDATE software
        SET software_name = $1, software_version = $2, developer_name = $3, description = $4, version = version + 1
        WHERE id = $5 AND version = $6
        RETURNING version
    "#,
        software.software_name,
        software.software_version,
        software.developer_name,
        software.description,
        software_id,
        software.version
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
