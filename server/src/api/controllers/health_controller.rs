use axum::http::StatusCode;

#[tracing::instrument(name = "health check")]
pub async fn api_health_check() -> StatusCode {
    StatusCode::NO_CONTENT
}
