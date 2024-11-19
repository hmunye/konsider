use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::json;

use crate::api::services::get_all_software;
use crate::api::utils::{Json, QueryExtractor, Token};
use crate::server::ServerState;
use crate::Result;

#[tracing::instrument(
    name = "get all software", 
    // Any values in 'skip' won't be included in logs
    skip(token, query_params, state),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_get_all_software(
    Token(token): Token,
    QueryExtractor(query_params): QueryExtractor,
    State(state): State<ServerState>,
) -> Result<impl IntoResponse> {
    tracing::Span::current().record("request_initiator", tracing::field::display(&token.sub));

    let (software, metadata) = get_all_software(query_params.0, &state.db_pool).await?;

    let response_body = json!({
        "metadata": if metadata.total_records == 0 {
            json!({})
        } else {
            json!(metadata)
        },
        "software": software
    });

    Ok((StatusCode::OK, Json(response_body)))
}
