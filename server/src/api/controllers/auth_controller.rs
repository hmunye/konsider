use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::Result;

#[derive(Debug, Deserialize, Serialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

pub async fn api_login(Json(payload): Json<Credentials>) -> Result<impl IntoResponse> {
    let response_body = json!({
        "received_payload": payload
    });

    Ok((StatusCode::OK, Json(response_body)))
}
