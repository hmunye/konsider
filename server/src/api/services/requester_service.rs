use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::api::controllers::UpdateRequesterPayload;
use crate::api::models::Requester;
use crate::api::repositories::{
    delete_requester, fetch_all_requesters, fetch_requester_by_id, insert_requester,
    update_requester,
};
use crate::api::utils::{Metadata, QueryParams};
use crate::{Error, Result};

#[tracing::instrument(name = "getting all requesters", skip(query_params, db_pool))]
pub async fn get_all_requesters(
    query_params: QueryParams,
    db_pool: &PgPool,
) -> Result<(Vec<Value>, Metadata)> {
    let sort_safe_list = [
        "name".to_string(),
        "email".to_string(),
        "department".to_string(),
        "-name".to_string(),
        "-email".to_string(),
        "-department".to_string(),
    ];

    let filter_safe_list = [
        "name".to_string(),
        "email".to_string(),
        "department".to_string(),
    ];

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

    let (requesters, metadata) = fetch_all_requesters(
        sort_column,
        sort_direction,
        page,
        per_page,
        filter_field,
        filter_value,
        db_pool,
    )
    .await?;

    let wrapped_requesters: Vec<Value> = requesters
        .into_iter()
        .map(|requester| {
            json!({
                "requester": requester
            })
        })
        .collect();

    Ok((wrapped_requesters, metadata))
}

#[tracing::instrument(name = "creating requester", skip(payload, db_pool))]
pub async fn create_requester(payload: &Requester, db_pool: &PgPool) -> Result<()> {
    insert_requester(payload, db_pool).await
}

#[tracing::instrument(name = "remove requester", skip(requester_id, db_pool))]
pub async fn remove_requester(requester_id: Uuid, db_pool: &PgPool) -> Result<()> {
    delete_requester(requester_id, db_pool).await
}

#[tracing::instrument(name = "update requester", skip(payload, requester_id, db_pool))]
pub async fn update_requester_details(
    payload: UpdateRequesterPayload,
    requester_id: Uuid,
    db_pool: &PgPool,
) -> Result<()> {
    // Fetch requester from database if a record exists
    let mut requester = fetch_requester_by_id(requester_id, db_pool).await?;

    let mut fields_updated = false;

    // Apply any updates to the `Requester` entity locally
    if let Some(name) = &payload.name {
        requester.name = name.clone();
        fields_updated = true;
    }

    if let Some(email) = &payload.email {
        requester.email = email.clone();
        fields_updated = true;
    }

    if let Some(department) = &payload.department {
        requester.department = department.clone();
        fields_updated = true;
    }

    // Return an error if no fields were updated
    if !fields_updated {
        return Err(Error::NoUpdatesProvidedError);
    }

    requester.parse()?;

    update_requester(requester, requester_id, db_pool).await
}
