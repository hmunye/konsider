use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use uuid::Uuid;

use crate::model::UserDTO;
use crate::server::AppState;
use crate::Result;

use super::fetch_user_by_id;

// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(
    name = "fetching user by id", 
    // Any values in 'skip' won't be included in logs
    skip(state, user_id),
    fields(
        user_id = tracing::field::Empty,
    )
)]
pub async fn api_get_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    tracing::Span::current().record("user_id", tracing::field::display(&user_id));

    let user = fetch_user_by_id(&state, &user_id).await?;

    // Using Data Transfer Object (DTO) to return only necessary fields, avoiding any sensitive
    // fields such as password_hash
    let user_dto = UserDTO::from(&user);

    let response_body = json!({
        "user": user_dto
    });

    Ok((StatusCode::OK, Json(response_body)))
}
