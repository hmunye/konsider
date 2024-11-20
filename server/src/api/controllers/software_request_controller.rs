use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use serde_json::json;

use crate::api::models::SoftwareRequest;
use crate::api::services::{
    create_software_request, get_all_software_requests, remove_software_request,
    update_software_request_details,
};
use crate::api::utils::{Json, Path, QueryExtractor, Token};
use crate::server::ServerState;
use crate::Result;

#[tracing::instrument(
    name = "get all software_requests", 
    // Any values in 'skip' won't be included in logs
    skip(token, query_params, state),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_get_all_software_requests(
    Token(token): Token,
    QueryExtractor(query_params): QueryExtractor,
    State(state): State<ServerState>,
) -> Result<impl IntoResponse> {
    tracing::Span::current().record("request_initiator", tracing::field::display(&token.sub));

    let (software_requests, metadata) =
        get_all_software_requests(query_params.0, &state.db_pool).await?;

    let response_body = json!({
        "metadata": if metadata.total_records == 0 {
            json!({})
        } else {
            json!(metadata)
        },
        "software_requests": software_requests
    });

    Ok((StatusCode::OK, Json(response_body)))
}

#[tracing::instrument(
    name = "create software request", 
    // Any values in 'skip' won't be included in logs
    skip(token, state, payload),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_create_software_request(
    Token(token): Token,
    State(state): State<ServerState>,
    Json(payload): Json<SoftwareRequest>,
) -> Result<StatusCode> {
    tracing::Span::current().record("request_initiator", tracing::field::display(&token.sub));

    payload.parse()?;

    create_software_request(&payload, &state.db_pool).await?;

    Ok(StatusCode::CREATED)
}

#[tracing::instrument(
    name = "delete software request", 
    // Any values in 'skip' won't be included in logs
    skip(token, request_id, state),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_delete_software_request(
    Token(token): Token,
    Path(request_id): Path<uuid::Uuid>,
    State(state): State<ServerState>,
) -> Result<StatusCode> {
    tracing::Span::current().record("request_initiator", tracing::field::display(&token.sub));

    remove_software_request(request_id, &state.db_pool).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Deserialize)]
pub struct UpdateSoftwareRequestPayload {
    pub td_request_id: Option<String>,
}

#[tracing::instrument(
    name = "update software request details", 
    // Any values in 'skip' won't be included in logs
    skip(token, request_id, state, payload),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_update_software_request(
    Token(token): Token,
    Path(request_id): Path<uuid::Uuid>,
    State(state): State<ServerState>,
    Json(payload): Json<UpdateSoftwareRequestPayload>,
) -> Result<StatusCode> {
    tracing::Span::current().record("request_initiator", tracing::field::display(&token.sub));

    update_software_request_details(payload, request_id, &state.db_pool).await?;

    Ok(StatusCode::NO_CONTENT)
}
