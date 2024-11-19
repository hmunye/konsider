use sqlx::PgPool;
use uuid::Uuid;

use crate::api::models::RequesterDTO;
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
