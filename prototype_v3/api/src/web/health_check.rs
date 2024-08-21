use axum::http::StatusCode;
use axum::response::IntoResponse;

#[tracing::instrument(name = "Health Check")]
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "status: ok")
}
