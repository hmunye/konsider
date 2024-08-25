use axum::{http::StatusCode, response::IntoResponse};

// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(name = "Health Check")]
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "status: ok").into_response()
}
