use axum::{http::StatusCode, Json};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StatusData {
    message: String,
    time: String,
}

pub async fn health_check() -> (StatusCode, Json<StatusData>) {
    let now: DateTime<Local> = Local::now();

    let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let response = StatusData {
        message: String::from("Konsider API Health Check"),
        time: formatted_time,
    };

    (StatusCode::OK, Json(response))
}
