use axum::http::StatusCode;

pub async fn api_health_check() -> StatusCode {
    StatusCode::NO_CONTENT
}
