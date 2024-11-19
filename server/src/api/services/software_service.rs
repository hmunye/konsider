use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::api::controllers::UpdateSoftwarePayload;
use crate::api::models::Software;
use crate::api::repositories::{
    delete_software, fetch_all_software, fetch_software_by_id, insert_software, update_software,
};
use crate::api::utils::{Metadata, QueryParams};
use crate::{Error, Result};

#[tracing::instrument(name = "getting all software", skip(query_params, db_pool))]
pub async fn get_all_software(
    query_params: QueryParams,
    db_pool: &PgPool,
) -> Result<(Vec<Value>, Metadata)> {
    let sort_safe_list = [
        "software_name".to_string(),
        "developer_name".to_string(),
        "-software_name".to_string(),
        "-developer_name".to_string(),
    ];

    let filter_safe_list = ["software_name".to_string(), "developer_name".to_string()];

    query_params.parse(&sort_safe_list, &filter_safe_list)?;

    let page = query_params.page.unwrap_or(1);
    let per_page = query_params.per_page.unwrap_or(10);

    let (sort_column, sort_direction) = match query_params
        .sort
        .unwrap_or("id".to_string())
        .strip_prefix("-")
    {
        Some(sort_column) => (sort_column.to_string(), "DESC".to_string()),
        None => ("id".to_string(), "ASC".to_string()),
    };

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

    let (software, metadata) = fetch_all_software(
        sort_column,
        sort_direction,
        page,
        per_page,
        filter_field,
        filter_value,
        db_pool,
    )
    .await?;

    let wrapped_software: Vec<Value> = software
        .into_iter()
        .map(|software| {
            json!({
                "software": software
            })
        })
        .collect();

    Ok((wrapped_software, metadata))
}

#[tracing::instrument(name = "creating software", skip(payload, db_pool))]
pub async fn create_software(payload: &Software, db_pool: &PgPool) -> Result<()> {
    insert_software(payload, db_pool).await
}

#[tracing::instrument(name = "removing software", skip(software_id, db_pool))]
pub async fn remove_software(software_id: Uuid, db_pool: &PgPool) -> Result<()> {
    delete_software(software_id, db_pool).await
}

#[tracing::instrument(name = "updating software", skip(payload, software_id, db_pool))]
pub async fn update_software_details(
    payload: UpdateSoftwarePayload,
    software_id: Uuid,
    db_pool: &PgPool,
) -> Result<()> {
    // Fetch software from database if a record exists
    let mut software = fetch_software_by_id(software_id, db_pool).await?;

    let mut fields_updated = false;

    // Apply any updates to the `Software` entity locally
    if let Some(software_name) = &payload.software_name {
        software.software_name = software_name.clone();
        fields_updated = true;
    }

    if let Some(software_version) = &payload.software_version {
        software.software_version = software_version.clone();
        fields_updated = true;
    }

    if let Some(developer_name) = &payload.developer_name {
        software.developer_name = developer_name.clone();
        fields_updated = true;
    }

    if let Some(description) = &payload.description {
        software.description = description.clone();
        fields_updated = true;
    }

    // Return an error if no fields were updated
    if !fields_updated {
        return Err(Error::NoUpdatesProvidedError);
    }

    software.parse()?;

    update_software(software, software_id, db_pool).await
}
