use serde_json::{json, Value};
use sqlx::PgPool;

use crate::api::controllers::UpdateSoftwareRequestPayload;
use crate::api::repositories::{
    delete_software_request, fetch_all_software_requests, fetch_software_request_by_id,
    insert_software_request, update_software_request,
};
use crate::api::utils::{Metadata, QueryParams};
use crate::api::SoftwareRequest;
use crate::{Error, Result};

#[tracing::instrument(name = "getting all software_requests", skip(query_params, db_pool))]
pub async fn get_all_software_requests(
    query_params: QueryParams,
    db_pool: &PgPool,
) -> Result<(Vec<Value>, Metadata)> {
    let sort_safe_list = [];

    let filter_safe_list = [
        "td_request_id".to_string(),
        "software_name".to_string(),
        "requester_email".to_string(),
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

    let (software_requests, metadata) =
        fetch_all_software_requests(page, per_page, filter_field, filter_value, db_pool).await?;

    let wrapped_software_requests: Vec<Value> = software_requests
        .into_iter()
        .map(|software_request| {
            json!({
                "software_request": software_request
            })
        })
        .collect();

    Ok((wrapped_software_requests, metadata))
}

#[tracing::instrument(name = "creating software request", skip(payload, db_pool))]
pub async fn create_software_request(payload: &SoftwareRequest, db_pool: &PgPool) -> Result<()> {
    insert_software_request(payload, db_pool).await
}

#[tracing::instrument(name = "removing software reqeust", skip(request_id, db_pool))]
pub async fn remove_software_request(request_id: uuid::Uuid, db_pool: &PgPool) -> Result<()> {
    delete_software_request(request_id, db_pool).await
}

#[tracing::instrument(name = "updating software request", skip(payload, request_id, db_pool))]
pub async fn update_software_request_details(
    payload: UpdateSoftwareRequestPayload,
    request_id: uuid::Uuid,
    db_pool: &PgPool,
) -> Result<()> {
    // Fetch software request from database if a record exists
    let mut software_request = fetch_software_request_by_id(request_id, db_pool).await?;

    let mut fields_updated = false;

    // Apply any updates to the `SoftwareRequest` entity locally
    if let Some(td_request_id) = &payload.td_request_id {
        software_request.td_request_id = td_request_id.clone();
        fields_updated = true;
    }

    // Return an error if no fields were updated
    if !fields_updated {
        return Err(Error::NoUpdatesProvidedError);
    }

    software_request.parse()?;

    update_software_request(software_request, request_id, db_pool).await
}
