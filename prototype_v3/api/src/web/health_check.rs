use axum::http::StatusCode;

#[tracing::instrument(name = "Health Check")]
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
