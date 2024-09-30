use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use uuid::Uuid;

use crate::model::{TypedSession, UserDTO};
use crate::server::AppState;
use crate::web::users::fetch_user_by_id;
use crate::Result;

// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(
    name = "fetching user by id", 
    // Any values in 'skip' won't be included in logs
    skip(state, session),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_get_user(
    State(state): State<AppState>,
    session: TypedSession,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    if let Some(current_user_id) = session.get_user_id().await? {
        tracing::Span::current().record(
            "request_initiator",
            tracing::field::display(&current_user_id),
        );
    }

    let user = fetch_user_by_id(&state, &user_id).await?;

    // Using Data Transfer Object (DTO) to return only necessary fields, avoiding any sensitive
    // fields such as password_hash
    let user_dto = UserDTO::from(&user);

    let response_body = json!({
        "user": user_dto
    });

    Ok((StatusCode::OK, Json(response_body)))
}
