use sqlx::PgPool;
use uuid::Uuid;

use crate::api::utils::Metadata;
use crate::api::{RequesterDTO, SoftwareDTO, SoftwareRequestDTO};
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

    // Build the query with joins to get data from related tables
    let query = if let (Some(field), Some(_)) = (filter_field.as_ref(), filter_value.as_ref()) {
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
            field, limit, offset
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
