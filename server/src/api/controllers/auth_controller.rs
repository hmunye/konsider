use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use secrecy::SecretString;
use serde::Deserialize;
use serde_json::json;

use crate::api::services::validate_credentials;
use crate::api::Json;
use crate::server::ServerState;
use crate::Result;

#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: SecretString,
}

#[tracing::instrument(
    name = "user login", 
    skip(payload, state),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_login(
    State(state): State<ServerState>,
    Json(payload): Json<Credentials>,
) -> Result<impl IntoResponse> {
    let (user_id, _user_role) = validate_credentials(&state.db_pool, payload).await?;

    tracing::Span::current().record("request_initiator", tracing::field::display(&user_id));

    let response_body = json!({
        "message": "login good"
    });

    Ok((StatusCode::OK, Json(response_body)))
}
