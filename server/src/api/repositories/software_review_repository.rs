use sqlx::PgPool;
use uuid::Uuid;

use crate::api::models::{
    RequesterDTO, ReviewOptions, SoftwareDTO, SoftwareRequestDTO, SoftwareReview,
    SoftwareReviewDTO, SoftwareReviewPayload, UserDTO, UserRole,
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
            id: record.id,
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
            exported: Some(record.exported),
            review_notes: Some(record.review_notes),
            created_at: Some(record.created_at),
        })
        .collect();

    let metadata = Metadata::calculate_metadata(total_records, page, per_page);

    Ok((software_reviews_records, metadata))
}

#[tracing::instrument(
    name = "fetching software review by id from database",
    skip(review_id, db_pool)
)]
pub async fn fetch_software_review_by_id(
    review_id: Uuid,
    db_pool: &PgPool,
) -> Result<SoftwareReview> {
    let row = sqlx::query!(
        r#"
        SELECT 
            id, software_request_id, reviewer_id, 
            is_supported AS "is_supported: ReviewOptions", 
            is_current_version AS "is_current_version: ReviewOptions", 
            is_reputation_good AS "is_reputation_good: ReviewOptions", 
            is_installation_from_developer AS "is_installation_from_developer: ReviewOptions", 
            is_local_admin_required AS "is_local_admin_required: ReviewOptions", 
            is_connected_to_brockport_cloud AS "is_connected_to_brockport_cloud: ReviewOptions", 
            is_connected_to_cloud_services_or_client AS "is_connected_to_cloud_services_or_client: ReviewOptions", 
            is_security_or_optimization_software AS "is_security_or_optimization_software: ReviewOptions", 
            is_supported_by_current_os AS "is_supported_by_current_os: ReviewOptions", 
            exported, review_notes, created_at, updated_at, version
        FROM software_review
        WHERE id = $1
        "#,
        review_id
    )
    .fetch_optional(db_pool)
    .await
    .map_err(Error::from)?;

    match row {
        Some(row) => Ok(SoftwareReview {
            id: row.id,
            software_request_id: row.software_request_id,
            reviewer_id: row.reviewer_id,
            is_supported: row.is_supported,
            is_current_version: row.is_current_version,
            is_reputation_good: row.is_reputation_good,
            is_installation_from_developer: row.is_installation_from_developer,
            is_local_admin_required: row.is_local_admin_required,
            is_connected_to_brockport_cloud: row.is_connected_to_brockport_cloud,
            is_connected_to_cloud_services_or_client: row.is_connected_to_cloud_services_or_client,
            is_security_or_optimization_software: row.is_security_or_optimization_software,
            is_supported_by_current_os: row.is_supported_by_current_os,
            exported: row.exported,
            review_notes: row.review_notes,
            created_at: row.created_at,
            updated_at: row.updated_at,
            version: row.version,
        }),
        None => Err(Error::PgNotFoundError),
    }
}

#[tracing::instrument(
    name = "inserting software review into database",
    skip(payload, db_pool)
)]
pub async fn insert_software_review(
    payload: &SoftwareReviewPayload,
    reviewer_id: &Uuid,
    db_pool: &PgPool,
) -> Result<()> {
    let mut tx = db_pool.begin().await?;

    let software_id = match sqlx::query!(
        r#"
        INSERT INTO software (software_name, software_version, developer_name, description)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
        payload.software_request.software.software_name,
        payload.software_request.software.software_version,
        payload.software_request.software.developer_name,
        payload.software_request.software.description,
    )
    .fetch_optional(&mut *tx)
    .await
    {
        Ok(Some(row)) => Ok(row.id),
        Ok(None) => Err(Error::PgNotFoundError),
        Err(err) => match err.as_database_error().and_then(|db_err| db_err.code()) {
            Some(code) if code == "23505" => Err(Error::PgRecordExists),
            _ => Err(Error::from(err)),
        },
    };

    let requester_id = match sqlx::query!(
        r#"
        INSERT INTO requester (name, email, department)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        payload.software_request.requester.name,
        payload.software_request.requester.email,
        payload.software_request.requester.department,
    )
    .fetch_optional(&mut *tx)
    .await
    {
        Ok(Some(row)) => Ok(row.id),
        Ok(None) => Err(Error::PgNotFoundError),
        Err(err) => match err.as_database_error().and_then(|db_err| db_err.code()) {
            Some(code) if code == "23505" => Err(Error::PgRecordExists),
            _ => Err(Error::from(err)),
        },
    };

    let software_request_id = match sqlx::query!(
        r#"
        INSERT INTO software_request (td_request_id, software_id, requester_id)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        payload.software_request.td_request_id,
        software_id?,
        requester_id?
    )
    .fetch_optional(&mut *tx)
    .await
    {
        Ok(Some(row)) => Ok(row.id),
        Ok(None) => Err(Error::PgNotFoundError),
        Err(err) => match err.as_database_error().and_then(|db_err| db_err.code()) {
            Some(code) if code == "23503" => Err(Error::PgKeyViolation),
            Some(code) if code == "23505" => Err(Error::PgRecordExists),
            _ => Err(Error::from(err)),
        },
    };

    let _ = match sqlx::query!(
        r#"
        INSERT INTO software_review (
            software_request_id, reviewer_id, 
            is_supported, is_current_version, is_reputation_good, 
            is_installation_from_developer, is_local_admin_required, 
            is_connected_to_brockport_cloud, is_connected_to_cloud_services_or_client, 
            is_security_or_optimization_software, is_supported_by_current_os, review_notes
        )
        VALUES (
            $1, $2, 
            $3, $4, $5, 
            $6, $7, 
            $8, $9, 
            $10, $11, $12
        )
        RETURNING id
        "#,
        software_request_id?,
        reviewer_id,
        payload.is_supported.clone() as ReviewOptions,
        payload.is_current_version.clone() as ReviewOptions,
        payload.is_reputation_good.clone() as ReviewOptions,
        payload.is_installation_from_developer.clone() as ReviewOptions,
        payload.is_local_admin_required.clone() as ReviewOptions,
        payload.is_connected_to_brockport_cloud.clone() as ReviewOptions,
        payload.is_connected_to_cloud_services_or_client.clone() as ReviewOptions,
        payload.is_security_or_optimization_software.clone() as ReviewOptions,
        payload.is_supported_by_current_os.clone() as ReviewOptions,
        payload.review_notes
    )
    .fetch_optional(&mut *tx)
    .await
    {
        Ok(Some(_)) => Ok(()),
        Ok(None) => Err(Error::PgNotFoundError),
        Err(err) => match err.as_database_error().and_then(|db_err| db_err.code()) {
            Some(code) if code == "23503" => Err(Error::PgKeyViolation),
            Some(code) if code == "23505" => Err(Error::PgRecordExists),
            _ => Err(Error::from(err)),
        },
    };

    tx.commit().await?;

    Ok(())
}

#[tracing::instrument(
    name = "deleting software review from database",
    skip(review_id, db_pool)
)]
pub async fn delete_software_review(review_id: Uuid, db_pool: &PgPool) -> Result<()> {
    match sqlx::query!(
        r#"
        DELETE FROM software_review
        WHERE id = $1
        RETURNING id
        "#,
        review_id,
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

#[tracing::instrument(
    name = "updating software review details in database",
    skip(software_review, review_id, db_pool)
)]
pub async fn update_software_review(
    software_review: SoftwareReview,
    review_id: Uuid,
    db_pool: &PgPool,
) -> Result<()> {
    let result = sqlx::query!(
        r#"
        UPDATE software_review
        SET 
            is_supported = $1,
            is_current_version = $2,
            is_reputation_good = $3,
            is_installation_from_developer = $4,
            is_local_admin_required = $5,
            is_connected_to_brockport_cloud = $6,
            is_connected_to_cloud_services_or_client = $7,
            is_security_or_optimization_software = $8,
            is_supported_by_current_os = $9,
            review_notes = $10,
            version = version + 1
        WHERE id = $11 AND version = $12
        RETURNING version
    "#,
        software_review.is_supported.clone() as ReviewOptions,
        software_review.is_current_version.clone() as ReviewOptions,
        software_review.is_reputation_good.clone() as ReviewOptions,
        software_review.is_installation_from_developer.clone() as ReviewOptions,
        software_review.is_local_admin_required.clone() as ReviewOptions,
        software_review.is_connected_to_brockport_cloud.clone() as ReviewOptions,
        software_review
            .is_connected_to_cloud_services_or_client
            .clone() as ReviewOptions,
        software_review.is_security_or_optimization_software.clone() as ReviewOptions,
        software_review.is_supported_by_current_os.clone() as ReviewOptions,
        software_review.review_notes,
        review_id,
        software_review.version
    )
    .fetch_optional(db_pool)
    .await;

    match result {
        Ok(Some(_)) => Ok(()),
        Ok(None) => Err(Error::PgNotFoundError),
        Err(err) => match err.as_database_error().and_then(|db_err| db_err.code()) {
            Some(code) if code == "23505" => Err(Error::PgRecordExists),
            _ => Err(Error::from(err)),
        },
    }
}

#[tracing::instrument(
    name = "updating exported column for software review in database",
    skip(review_id, db_pool)
)]
pub async fn update_software_review_exported(
    review_id: &Uuid,
    review_version: i32,
    db_pool: &PgPool,
) -> Result<()> {
    match sqlx::query!(
        r#"
        UPDATE software_review
        SET 
            exported = TRUE,
            version = version + 1
        WHERE id = $1 AND version = $2
        RETURNING version
        "#,
        review_id,
        review_version
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
    name = "fetching software request and reviewer details from software review",
    skip(review_id, db_pool)
)]
pub async fn fetch_software_review_details(
    review_id: Uuid,
    db_pool: &PgPool,
) -> Result<(SoftwareRequestDTO, UserDTO)> {
    let row = sqlx::query!(
        r#"
        SELECT 
            r.id AS software_request_id,
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
            u.id AS reviewer_id,
            u.name AS reviewer_name,
            u.email AS reviewer_email,
            u.role AS "role: UserRole",
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
            sr.id = $1
        "#,
        review_id
    )
    .fetch_optional(db_pool)
    .await
    .map_err(Error::from)?;

    match row {
        Some(row) => {
            let software_request_dto = SoftwareRequestDTO {
                id: Some(row.software_request_id),
                td_request_id: row.td_request_id,
                software: SoftwareDTO {
                    id: Some(row.software_id),
                    software_name: row.software_name,
                    software_version: row.software_version,
                    developer_name: row.developer_name,
                    description: row.description,
                    created_at: row.software_created_at,
                },
                requester: RequesterDTO {
                    id: Some(row.requester_id),
                    name: row.requester_name,
                    email: row.requester_email,
                    department: row.requester_department,
                    created_at: row.requester_created_at,
                },
                created_at: row.software_request_created_at,
            };

            let user_dto = UserDTO {
                id: Some(row.reviewer_id),
                name: row.reviewer_name,
                email: row.reviewer_email,
                // unwrap here should be safe
                role: row.role.unwrap(),
                created_at: row.reviewer_created_at,
            };

            Ok((software_request_dto, user_dto))
        }
        None => Err(Error::PgNotFoundError),
    }
}
