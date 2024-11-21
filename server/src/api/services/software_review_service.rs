use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::api::controllers::UpdateSoftwareReviewPayload;
use crate::api::models::SoftwareReviewPayload;
use crate::api::repositories::{
    delete_software_review, fetch_all_software_reviews, fetch_software_review_by_id,
    insert_software_review, update_software_review,
};
use crate::api::utils::{Metadata, QueryParams};
use crate::{Error, Result};

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
    payload: &SoftwareReviewPayload,
    reviewer_id: &Uuid,
    db_pool: &PgPool,
) -> Result<()> {
    insert_software_review(payload, reviewer_id, db_pool).await
}

#[tracing::instrument(name = "removing software review", skip(review_id, db_pool))]
pub async fn remove_software_review(review_id: uuid::Uuid, db_pool: &PgPool) -> Result<()> {
    delete_software_review(review_id, db_pool).await
}

#[tracing::instrument(
    name = "updating software review details",
    skip(payload, review_id, db_pool)
)]
pub async fn update_software_review_details(
    payload: UpdateSoftwareReviewPayload,
    review_id: Uuid,
    db_pool: &PgPool,
) -> Result<()> {
    // Fetch the existing review details from the database
    let mut software_review = fetch_software_review_by_id(review_id, db_pool).await?;

    let mut fields_updated = false;

    // Apply any updates to the `SoftwareReview` entity locally
    if let Some(is_supported) = payload.is_supported {
        software_review.is_supported = is_supported;
        fields_updated = true;
    }

    if let Some(is_current_version) = payload.is_current_version {
        software_review.is_current_version = is_current_version;
        fields_updated = true;
    }

    if let Some(is_reputation_good) = payload.is_reputation_good {
        software_review.is_reputation_good = is_reputation_good;
        fields_updated = true;
    }

    if let Some(is_installation_from_developer) = payload.is_installation_from_developer {
        software_review.is_installation_from_developer = is_installation_from_developer;
        fields_updated = true;
    }

    if let Some(is_local_admin_required) = payload.is_local_admin_required {
        software_review.is_local_admin_required = is_local_admin_required;
        fields_updated = true;
    }

    if let Some(is_connected_to_brockport_cloud) = payload.is_connected_to_brockport_cloud {
        software_review.is_connected_to_brockport_cloud = is_connected_to_brockport_cloud;
        fields_updated = true;
    }

    if let Some(is_connected_to_cloud_services_or_client) =
        payload.is_connected_to_cloud_services_or_client
    {
        software_review.is_connected_to_cloud_services_or_client =
            is_connected_to_cloud_services_or_client;
        fields_updated = true;
    }

    if let Some(is_security_or_optimization_software) = payload.is_security_or_optimization_software
    {
        software_review.is_security_or_optimization_software = is_security_or_optimization_software;
        fields_updated = true;
    }

    if let Some(is_supported_by_current_os) = payload.is_supported_by_current_os {
        software_review.is_supported_by_current_os = is_supported_by_current_os;
        fields_updated = true;
    }

    if let Some(review_notes) = payload.review_notes {
        software_review.review_notes = Some(review_notes);
        fields_updated = true;
    }

    // Return an error if no fields were updated
    if !fields_updated {
        return Err(Error::NoUpdatesProvidedError);
    }

    software_review.parse()?;

    update_software_review(software_review, review_id, db_pool).await
}
