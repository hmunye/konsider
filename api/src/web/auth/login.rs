use axum::extract::{self, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

use crate::model::{TypedSession, UserDTO};
use crate::server::AppState;
use crate::web::auth::{validate_credentials, Credentials};
use crate::web::users::fetch_user_by_id;
use crate::Result;

// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(
    name = "logging in user", 
    // Any values in 'skip' won't be included in logs
    skip(state, session, payload),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_login(
    State(state): State<AppState>,
    session: TypedSession,
    extract::Json(payload): extract::Json<Credentials>,
) -> Result<impl IntoResponse> {
    let user_id = validate_credentials(&state, payload).await?;

    tracing::Span::current().record("request_initiator", tracing::field::display(&user_id));

    // Rotating session id prevents session fixation attacks
    session.cycle().await?;

    // Create session with user id
    session.insert_user_id(user_id).await?;

    let user = fetch_user_by_id(&state, &user_id).await?;

    // Using Data Transfer Object (DTO) to return only necessary fields, avoiding any sensitive
    // fields such as password_hash
    let user_dto = UserDTO::from(&user);

    let response_body = json!({
        "user": user_dto
    });

    Ok((StatusCode::OK, Json(response_body)))
}
