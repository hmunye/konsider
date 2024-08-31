use axum::http::StatusCode;
use axum::response::IntoResponse;

// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(name = "health check")]
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "status: ok").into_response()
}
