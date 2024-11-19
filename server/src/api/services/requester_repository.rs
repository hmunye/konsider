use serde_json::{json, Value};
use sqlx::PgPool;

use crate::api::repositories::fetch_all_requesters;
use crate::api::utils::{Metadata, QueryParams};
use crate::Result;

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
        .map(|user| {
            json!({
                "user": user
            })
        })
        .collect();

    Ok((wrapped_requesters, metadata))
}
