use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use serde_json::json;

use crate::api::models::{ReviewOptions, SoftwareReviewPayload};
use crate::api::services::{
    create_software_review, get_all_software_reviews, remove_software_review,
    update_software_review_details,
};
use crate::api::utils::{Json, Path, QueryExtractor, Token};
use crate::server::ServerState;
use crate::Result;

#[tracing::instrument(
    name = "get all software reviews", 
    // Any values in 'skip' won't be included in logs
    skip(token, query_params, state),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_get_all_software_reviews(
    Token(token): Token,
    QueryExtractor(query_params): QueryExtractor,
    State(state): State<ServerState>,
) -> Result<impl IntoResponse> {
    tracing::Span::current().record("request_initiator", tracing::field::display(&token.sub));

    let (software_reviews, metadata) =
        get_all_software_reviews(query_params.0, &state.db_pool).await?;

    let response_body = json!({
        "metadata": if metadata.total_records == 0 {
            json!({})
        } else {
            json!(metadata)
        },
        "software_reviews": software_reviews
    });

    Ok((StatusCode::OK, Json(response_body)))
}

#[tracing::instrument(
    name = "create software review", 
    // Any values in 'skip' won't be included in logs
    skip(token, state, payload),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_create_software_review(
    Token(token): Token,
    State(state): State<ServerState>,
    Json(payload): Json<SoftwareReviewPayload>,
) -> Result<StatusCode> {
    tracing::Span::current().record("request_initiator", tracing::field::display(&token.sub));

    payload.parse()?;
    payload.software_request.parse()?;
    payload.software_request.software.parse()?;
    payload.software_request.requester.parse()?;

    create_software_review(&payload, &token.sub, &state.db_pool).await?;

    Ok(StatusCode::CREATED)
}

#[tracing::instrument(
    name = "delete software review", 
    // Any values in 'skip' won't be included in logs
    skip(token, review_id, state),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_delete_software_review(
    Token(token): Token,
    Path(review_id): Path<uuid::Uuid>,
    State(state): State<ServerState>,
) -> Result<StatusCode> {
    tracing::Span::current().record("request_initiator", tracing::field::display(&token.sub));

    remove_software_review(review_id, &state.db_pool).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Deserialize)]
pub struct UpdateSoftwareReviewPayload {
    pub is_supported: Option<ReviewOptions>,
    pub is_current_version: Option<ReviewOptions>,
    pub is_reputation_good: Option<ReviewOptions>,
    pub is_installation_from_developer: Option<ReviewOptions>,
    pub is_local_admin_required: Option<ReviewOptions>,
    pub is_connected_to_brockport_cloud: Option<ReviewOptions>,
    pub is_connected_to_cloud_services_or_client: Option<ReviewOptions>,
    pub is_security_or_optimization_software: Option<ReviewOptions>,
    pub is_supported_by_current_os: Option<ReviewOptions>,
    pub review_notes: Option<String>,
}

#[tracing::instrument(
    name = "update software review details", 
    // Any values in 'skip' won't be included in logs
    skip(token, review_id, state, payload),
    fields(
        review_initiator = tracing::field::Empty,
    )
)]
pub async fn api_update_software_review(
    Token(token): Token,
    Path(review_id): Path<uuid::Uuid>,
    State(state): State<ServerState>,
    Json(payload): Json<UpdateSoftwareReviewPayload>,
) -> Result<StatusCode> {
    tracing::Span::current().record("review_initiator", tracing::field::display(&token.sub));

    update_software_review_details(payload, review_id, &state.db_pool).await?;

    Ok(StatusCode::NO_CONTENT)
}
