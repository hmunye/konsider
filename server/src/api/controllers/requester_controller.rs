use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use serde_json::json;

use crate::api::models::Requester;
use crate::api::services::{
    create_requester, get_all_requesters, remove_requester, update_requester_details,
};
use crate::api::utils::{Json, Path, QueryExtractor, Token};
use crate::server::ServerState;
use crate::Result;

#[tracing::instrument(
    name = "get all requesters", 
    // Any values in 'skip' won't be included in logs
    skip(token, query_params, state),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_get_all_requesters(
    Token(token): Token,
    QueryExtractor(query_params): QueryExtractor,
    State(state): State<ServerState>,
) -> Result<impl IntoResponse> {
    tracing::Span::current().record("request_initiator", tracing::field::display(&token.sub));

    let (requesters, metadata) = get_all_requesters(query_params.0, &state.db_pool).await?;

    let response_body = json!({
        "metadata": if metadata.total_records == 0 {
            json!({})
        } else {
            json!(metadata)
        },
        "requesters": requesters
    });

    Ok((StatusCode::OK, Json(response_body)))
}

#[tracing::instrument(
    name = "create requester", 
    // Any values in 'skip' won't be included in logs
    skip(token, state, payload),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_create_requester(
    Token(token): Token,
    State(state): State<ServerState>,
    Json(payload): Json<Requester>,
) -> Result<StatusCode> {
    tracing::Span::current().record("request_initiator", tracing::field::display(&token.sub));

    payload.parse()?;

    let _ = create_requester(&payload, &state.db_pool).await?;

    Ok(StatusCode::CREATED)
}

#[tracing::instrument(
    name = "delete requester", 
    // Any values in 'skip' won't be included in logs
    skip(token, requester_id, state),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_delete_requester(
    Token(token): Token,
    Path(requester_id): Path<uuid::Uuid>,
    State(state): State<ServerState>,
) -> Result<StatusCode> {
    tracing::Span::current().record("request_initiator", tracing::field::display(&token.sub));

    remove_requester(requester_id, &state.db_pool).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Deserialize)]
pub struct UpdateRequesterPayload {
    pub name: Option<String>,
    pub email: Option<String>,
    pub department: Option<String>,
}

#[tracing::instrument(
    name = "update requester details", 
    // Any values in 'skip' won't be included in logs
    skip(token, requester_id, state, payload),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_update_requester(
    Token(token): Token,
    Path(requester_id): Path<uuid::Uuid>,
    State(state): State<ServerState>,
    Json(payload): Json<UpdateRequesterPayload>,
) -> Result<StatusCode> {
    tracing::Span::current().record("request_initiator", tracing::field::display(&token.sub));

    update_requester_details(payload, requester_id, &state.db_pool).await?;

    Ok(StatusCode::NO_CONTENT)
}
