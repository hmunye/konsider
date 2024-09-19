use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::{json, Value};

use crate::model::{QueryExtractor, TypedSession};
use crate::server::AppState;
use crate::web::users::fetch_users;
use crate::Result;

// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(
    name = "fetching all users", 
    // Any values in 'skip' won't be included in logs
    skip(state, session, query),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_get_all_users(
    State(state): State<AppState>,
    session: TypedSession,
    query: QueryExtractor,
) -> Result<impl IntoResponse> {
    if let Some(current_user_id) = session.get_user_id().await? {
        tracing::Span::current().record(
            "request_initiator",
            tracing::field::display(&current_user_id),
        );
    }

    let sort_safe_list = [
        "name".to_string(),
        "email".to_string(),
        "role".to_string(),
        "-name".to_string(),
        "-email".to_string(),
        "-role".to_string(),
    ];

    let query_params = query.0.parse(&sort_safe_list)?;

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

    let (users, metadata) =
        fetch_users(&state, sort_column, sort_direction, page, per_page).await?;

    let wrapped_users: Vec<Value> = users
        .into_iter()
        .map(|user| {
            json!({
                "user": user
            })
        })
        .collect();

    let response_body = json!({
         "metadata": if metadata.total_records == 0 {
            json!({})
        } else {
            json!(metadata)
        },
        "users": wrapped_users
    });

    Ok((StatusCode::OK, Json(response_body)))
}
