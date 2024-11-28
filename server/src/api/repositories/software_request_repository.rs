use sqlx::PgPool;
use uuid::Uuid;

use crate::api::utils::Metadata;
use crate::api::{RequesterDTO, SoftwareDTO, SoftwareRequest, SoftwareRequestDTO};
use crate::{Error, Result};

#[derive(Debug, sqlx::FromRow)]
struct SoftwareRequestRecordCount {
    count: i64,
    id: Uuid,
    td_request_id: String,
    created_at: chrono::DateTime<chrono::Utc>,
    software_id: Uuid,
    software_name: String,
    software_version: String,
    developer_name: String,
    description: String,
    software_created_at: chrono::DateTime<chrono::Utc>,
    requester_id: Uuid,
    requester_name: String,
    requester_email: String,
    requester_department: String,
    requester_created_at: chrono::DateTime<chrono::Utc>,
}

#[tracing::instrument(
    name = "fetching all software_requests from database",
    skip(page, per_page, filter_field, filter_value, db_pool)
)]
pub async fn fetch_all_software_requests(
    page: usize,
    per_page: usize,
    filter_field: Option<String>,
    filter_value: Option<String>,
    db_pool: &PgPool,
) -> Result<(Vec<SoftwareRequestDTO>, Metadata)> {
    let limit = per_page;
    let offset = (page - 1) * per_page;

    // Mutate filter_field before query construction
    let (field, mut column) = (filter_field.clone(), None);

    if let Some(ref filter_field) = field {
        column = match filter_field.as_str() {
            "software_name" => Some("s.software_name"),
            "requester_email" => Some("r.email"),
            _ => Some("td_request_id"),
        };
    }

    // Build the query with joins to get data from related tables
    let query = if let (Some(_), Some(_)) = (filter_field.as_ref(), filter_value.as_ref()) {
        format!(
            r#"
        SELECT 
            count(*) OVER(),
            sr.id,
            sr.td_request_id,
            sr.created_at,
            s.id AS software_id,
            s.software_name,
            s.software_version,
            s.developer_name,
            s.description,
            s.created_at AS software_created_at,
            r.id AS requester_id,
            r.name AS requester_name,
            r.email AS requester_email,
            r.department AS requester_department,
            r.created_at AS requester_created_at
        FROM 
            software_request sr
        INNER JOIN 
            software s ON sr.software_id = s.id
        INNER JOIN 
            requester r ON sr.requester_id = r.id
        WHERE 
            (to_tsvector('simple', {}::TEXT) @@ plainto_tsquery('simple', $1))
        ORDER BY 
            sr.id ASC
        LIMIT {} OFFSET {}
        "#,
            column.unwrap_or_default(),
            limit,
            offset
        )
    } else {
        format!(
            r#"
        SELECT 
            count(*) OVER(),
            sr.id,
            sr.td_request_id,
            sr.created_at,
            s.id AS software_id,
            s.software_name,
            s.software_version,
            s.developer_name,
            s.description,
            s.created_at AS software_created_at,
            r.id AS requester_id,
            r.name AS requester_name,
            r.email AS requester_email,
            r.department AS requester_department,
            r.created_at AS requester_created_at
        FROM 
            software_request sr
        INNER JOIN 
            software s ON sr.software_id = s.id
        INNER JOIN 
            requester r ON sr.requester_id = r.id
        ORDER BY 
            sr.id ASC
        LIMIT {} OFFSET {}
        "#,
            limit, offset
        )
    };
    let query = sqlx::query_as::<_, SoftwareRequestRecordCount>(&query);

    // Bind the user-supplied value only if it exists
    let query = if let Some(value) = filter_value {
        query.bind(value)
    } else {
        query
    };

    let records = query.fetch_all(db_pool).await.map_err(Error::from)?;

    let total_records = records.first().map_or(0, |record| record.count);

    let software_requests_records: Vec<SoftwareRequestDTO> = records
        .into_iter()
        .map(|record| SoftwareRequestDTO {
            id: Some(record.id),
            td_request_id: record.td_request_id,
            software: SoftwareDTO {
                id: Some(record.software_id),
                software_name: record.software_name,
                software_version: record.software_version,
                developer_name: record.developer_name,
                description: record.description,
                created_at: Some(record.software_created_at),
            },
            requester: RequesterDTO {
                id: Some(record.requester_id),
                name: record.requester_name,
                email: record.requester_email,
                department: record.requester_department,
                created_at: Some(record.requester_created_at),
            },
            created_at: Some(record.created_at),
        })
        .collect();

    let metadata = Metadata::calculate_metadata(total_records, page, per_page);

    Ok((software_requests_records, metadata))
}

#[tracing::instrument(
    name = "fetching software request by id from database",
    skip(request_id, db_pool)
)]
pub async fn fetch_software_request_by_id(
    request_id: Uuid,
    db_pool: &PgPool,
) -> Result<SoftwareRequest> {
    let row = sqlx::query!(
        r#"
        SELECT id, td_request_id, software_id, requester_id, created_at, updated_at, version
        FROM software_request
        WHERE id = $1
        "#,
        request_id
    )
    .fetch_optional(db_pool)
    .await
    .map_err(Error::from)?;

    match row {
        Some(row) => Ok(SoftwareRequest {
            id: Some(row.id),
            td_request_id: row.td_request_id,
            software_id: row.software_id,
            requester_id: row.requester_id,
            created_at: row.created_at,
            updated_at: row.updated_at,
            version: row.version,
        }),
        None => Err(Error::PgNotFoundError),
    }
}

#[tracing::instrument(
    name = "inserting software request into database",
    skip(payload, db_pool)
)]
pub async fn insert_software_request(payload: &SoftwareRequest, db_pool: &PgPool) -> Result<Uuid> {
    match sqlx::query!(
        r#"
        INSERT INTO software_request (td_request_id, software_id, requester_id)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        payload.td_request_id,
        payload.software_id,
        payload.requester_id
    )
    .fetch_optional(db_pool)
    .await
    {
        Ok(Some(row)) => Ok(row.id),
        Ok(None) => Err(Error::PgNotFoundError),
        Err(err) => match err.as_database_error().and_then(|db_err| db_err.code()) {
            Some(code) if code == "23503" => Err(Error::PgKeyViolation),
            Some(code) if code == "23505" => Err(Error::PgRecordExists),
            _ => Err(Error::from(err)),
        },
    }
}

#[tracing::instrument(
    name = "deleting software request from database",
    skip(request_id, db_pool)
)]
pub async fn delete_software_request(request_id: Uuid, db_pool: &PgPool) -> Result<()> {
    match sqlx::query!(
        r#"
        DELETE FROM software_request
        WHERE id = $1
        RETURNING id
        "#,
        request_id,
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
    name = "updating software request details in database",
    skip(software_request, request_id, db_pool)
)]
pub async fn update_software_request(
    software_request: SoftwareRequest,
    request_id: Uuid,
    db_pool: &PgPool,
) -> Result<()> {
    match sqlx::query!(
        r#"
        UPDATE software_request
        SET td_request_id = $1, version = version + 1
        WHERE id = $2 AND version = $3
        RETURNING version
    "#,
        software_request.td_request_id,
        request_id,
        software_request.version
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
