use axum::{http::StatusCode, Json};
use chrono::{DateTime, Local};
use serde::Serialize;

#[derive(Serialize)]
pub struct StatusResponse {
    message: String,
    time: String,
    status: String,
}

pub async fn health_check() -> Json<StatusResponse> {
    let now: DateTime<Local> = Local::now();
    let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();

    Json(StatusResponse {
        message: String::from("Konsider API Health Check"),
        time: formatted_time,
        status: StatusCode::OK.to_string(),
    })
}
