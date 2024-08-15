use axum::extract;
use axum::http::StatusCode;
use axum::response::Response;
use serde::Deserialize;
use serde_json::json;

use crate::{Error, Result};

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    email: String,
    password: String,
}

pub async fn login_handler(
    extract::Json(payload): extract::Json<LoginPayload>,
) -> Result<Response<String>> {
    // TODO: Implement Database Authentication
    if payload.email != "test" || payload.password != "test" {
        return Err(Error::LoginFail);
    }

    // TODO: Get User ID

    // TODO: Set Cookie using User ID

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
                "success": true,
            })
            .to_string(),
        )
        .unwrap();

    Ok(response)
}
