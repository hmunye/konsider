use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::api::models::SoftwareReview;
use crate::api::repositories::{fetch_all_software_reviews, insert_software_review};
use crate::api::utils::{Metadata, QueryParams};
use crate::Result;

#[tracing::instrument(name = "getting all software reviews", skip(query_params, db_pool))]
pub async fn get_all_software_reviews(
    query_params: QueryParams,
    db_pool: &PgPool,
) -> Result<(Vec<Value>, Metadata)> {
    let sort_safe_list = [];

    let filter_safe_list = [
        "td_request_id".to_string(),
        "reviewer_email".to_string(),
        "requester_email".to_string(),
        "software_name".to_string(),
        "exported".to_string(),
    ];

    query_params.parse(&sort_safe_list, &filter_safe_list)?;

    let page = query_params.page.unwrap_or(1);
    let per_page = query_params.per_page.unwrap_or(10);

    let mut filter_field = None;
    let mut filter_value = None;

    if let Some(filter_str) = query_params.filter {
        let parts: Vec<&str> = filter_str.split(':').collect();

        if parts.len() == 2 {
            let field = parts[0].to_string();
            let value = parts[1].to_string();

            filter_field = Some(field);
            filter_value = Some(value);
        }
    }

    let (software_reviews, metadata) =
        fetch_all_software_reviews(page, per_page, filter_field, filter_value, db_pool).await?;

    let wrapped_software_reviews: Vec<Value> = software_reviews
        .into_iter()
        .map(|software_review| {
            json!({
                "software_review": software_review
            })
        })
        .collect();

    Ok((wrapped_software_reviews, metadata))
}

#[tracing::instrument(name = "creating software review", skip(payload, db_pool))]
pub async fn create_software_review(
    payload: &SoftwareReview,
    reviewer_id: &Uuid,
    db_pool: &PgPool,
) -> Result<()> {
    insert_software_review(payload, reviewer_id, db_pool).await
}
