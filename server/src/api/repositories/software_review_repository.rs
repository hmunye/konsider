use sqlx::PgPool;
use uuid::Uuid;

use crate::api::models::{
    RequesterDTO, ReviewOptions, SoftwareDTO, SoftwareRequestDTO, SoftwareReviewDTO, UserDTO,
    UserRole,
};
use crate::api::utils::Metadata;
use crate::{Error, Result};

#[derive(Debug, sqlx::FromRow)]
struct SoftwareReviewRecordCount {
    count: i64,
    id: Uuid,
    software_request_id: Uuid,
    td_request_id: String,
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
    reviewer_id: Uuid,
    reviewer_name: String,
    reviewer_email: String,
    reviewer_role: UserRole,
    reviewer_created_at: chrono::DateTime<chrono::Utc>,
    software_request_created_at: chrono::DateTime<chrono::Utc>,
    is_supported: ReviewOptions,
    is_current_version: ReviewOptions,
    is_reputation_good: ReviewOptions,
    is_installation_from_developer: ReviewOptions,
    is_local_admin_required: ReviewOptions,
    is_connected_to_brockport_cloud: ReviewOptions,
    is_connected_to_cloud_services_or_client: ReviewOptions,
    is_security_or_optimization_software: ReviewOptions,
    is_supported_by_current_os: ReviewOptions,
    exported: bool,
    review_notes: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[tracing::instrument(
    name = "fetching all software reviews from database",
    skip(page, per_page, filter_field, filter_value, db_pool)
)]
pub async fn fetch_all_software_reviews(
    page: usize,
    per_page: usize,
    filter_field: Option<String>,
    filter_value: Option<String>,
    db_pool: &PgPool,
) -> Result<(Vec<SoftwareReviewDTO>, Metadata)> {
    let limit = per_page;
    let offset = (page - 1) * per_page;

    // Mutate filter_field before query construction
    let (field, mut column) = (filter_field.clone(), None);

    if let Some(ref filter_field) = field {
        column = match filter_field.as_str() {
            "td_request_id" => Some("r.td_request_id"),
            "reviewer_email" => Some("u.email"),
            "requester_email" => Some("rq.email"),
            "software_name" => Some("s.software_name"),
            _ => Some("sr.exported"),
        };
    }

    // Build the query with joins to get data from related tables
    let query = if let (Some(_), Some(_)) = (filter_field.as_ref(), filter_value.as_ref()) {
        format!(
            r#"
            SELECT 
                count(*) OVER() AS count,
                sr.id,
                sr.software_request_id,
                sr.reviewer_id,
                sr.is_supported,
                sr.is_current_version,
                sr.is_reputation_good,
                sr.is_installation_from_developer,
                sr.is_local_admin_required,
                sr.is_connected_to_brockport_cloud,
                sr.is_connected_to_cloud_services_or_client,
                sr.is_security_or_optimization_software,
                sr.is_supported_by_current_os,
                sr.exported,
                sr.review_notes,
                sr.created_at,
                sr.updated_at,
                r.td_request_id,
                r.created_at AS software_request_created_at,
                s.id AS software_id,
                s.software_name,
                s.software_version,
                s.developer_name,
                s.description,
                s.created_at AS software_created_at,
                rq.id AS requester_id,
                rq.name AS requester_name,
                rq.email AS requester_email,
                rq.department AS requester_department,
                rq.created_at AS requester_created_at,
                u.name AS reviewer_name,
                u.email AS reviewer_email,
                u.role AS reviewer_role,
                u.created_at AS reviewer_created_at
            FROM 
                software_review sr
            INNER JOIN 
                software_request r ON sr.software_request_id = r.id
            INNER JOIN 
                software s ON r.software_id = s.id
            INNER JOIN 
                requester rq ON r.requester_id = rq.id
            INNER JOIN 
                user_account u ON sr.reviewer_id = u.id
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
                count(*) OVER() AS count,
                sr.id,
                sr.software_request_id,
                sr.reviewer_id,
                sr.is_supported,
                sr.is_current_version,
                sr.is_reputation_good,
                sr.is_installation_from_developer,
                sr.is_local_admin_required,
                sr.is_connected_to_brockport_cloud,
                sr.is_connected_to_cloud_services_or_client,
                sr.is_security_or_optimization_software,
                sr.is_supported_by_current_os,
                sr.exported,
                sr.review_notes,
                sr.created_at,
                sr.updated_at,
                r.td_request_id,
                r.created_at AS software_request_created_at,
                s.id AS software_id,
                s.software_name,
                s.software_version,
                s.developer_name,
                s.description,
                s.created_at AS software_created_at,
                rq.id AS requester_id,
                rq.name AS requester_name,
                rq.email AS requester_email,
                rq.department AS requester_department,
                rq.created_at AS requester_created_at,
                u.name AS reviewer_name,
                u.email AS reviewer_email,
                u.role AS reviewer_role,
                u.created_at AS reviewer_created_at
            FROM 
                software_review sr
            INNER JOIN 
                software_request r ON sr.software_request_id = r.id
            INNER JOIN 
                software s ON r.software_id = s.id
            INNER JOIN 
                requester rq ON r.requester_id = rq.id
            INNER JOIN 
                user_account u ON sr.reviewer_id = u.id
            ORDER BY 
                sr.id ASC
            LIMIT {} OFFSET {}
            "#,
            limit, offset
        )
    };
    let query = sqlx::query_as::<_, SoftwareReviewRecordCount>(&query);

    // Bind the user-supplied value only if it exists
    let query = if let Some(value) = filter_value {
        query.bind(value)
    } else {
        query
    };

    let records = query.fetch_all(db_pool).await.map_err(Error::from)?;

    let total_records = records.first().map_or(0, |record| record.count);

    let software_reviews_records: Vec<SoftwareReviewDTO> = records
        .into_iter()
        .map(|record| SoftwareReviewDTO {
            id: Some(record.id),
            software_request: SoftwareRequestDTO {
                id: Some(record.software_request_id),
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
                created_at: Some(record.software_request_created_at),
            },
            reviewer: UserDTO {
                id: Some(record.reviewer_id),
                name: record.reviewer_name,
                email: record.reviewer_email,
                role: record.reviewer_role,
                created_at: Some(record.reviewer_created_at),
            },
            is_supported: record.is_supported,
            is_current_version: record.is_current_version,
            is_reputation_good: record.is_reputation_good,
            is_installation_from_developer: record.is_installation_from_developer,
            is_local_admin_required: record.is_local_admin_required,
            is_connected_to_brockport_cloud: record.is_connected_to_brockport_cloud,
            is_connected_to_cloud_services_or_client: record
                .is_connected_to_cloud_services_or_client,
            is_security_or_optimization_software: record.is_security_or_optimization_software,
            is_supported_by_current_os: record.is_supported_by_current_os,
            exported: record.exported,
            review_notes: record.review_notes,
            created_at: Some(record.created_at),
        })
        .collect();

    let metadata = Metadata::calculate_metadata(total_records, page, per_page);

    Ok((software_reviews_records, metadata))
}
