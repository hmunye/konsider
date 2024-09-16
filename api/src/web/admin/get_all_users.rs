use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

use crate::server::AppState;
use crate::web::admin::fetch_all_users;
use crate::Result;

// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(
    name = "fetching all users", 
    // Any values in 'skip' won't be included in logs
    skip(state),
    fields(
        user_id = tracing::field::Empty,
    )
)]
pub async fn api_get_all_users(State(state): State<AppState>) -> Result<impl IntoResponse> {
    // tracing::Span::current().record("user_id", tracing::field::display(&user_id));

    let users = fetch_all_users(&state).await?;

    let response_body = json!({
        "users": users
    });

    Ok((StatusCode::OK, Json(response_body)))
}
