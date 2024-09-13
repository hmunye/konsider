use axum::extract::{Path, State};
use axum::http::StatusCode;
use uuid::Uuid;

use crate::server::AppState;
use crate::Result;

use crate::web::admin::delete_user;

// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(
    name = "deleting user", 
    // Any values in 'skip' won't be included in logs
    skip(state, user_id),
    fields(
        user_id = tracing::field::Empty,
    )
)]
pub async fn api_delete_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<StatusCode> {
    tracing::Span::current().record("user_id", tracing::field::display(&user_id));

    delete_user(&state, &user_id).await?;

    Ok(StatusCode::OK)
}
