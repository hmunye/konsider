use axum::response::{IntoResponse, Response};
use axum::{http::StatusCode, Json};
use serde_json::json;

use crate::error::ClientError;
use crate::Error;

// Modify responses before they are sent to the client
pub async fn main_response_mapper(res: Response) -> Response {
    let status_code = res.status();

    // Handle any 422 status codes with custom response, minimizing information disclosure
    // Ex. If payload to create user is missing 'role', it cannot properly desearialize it,
    // resulting in a 422 UNPROCESSABLE ENTITY status
    if status_code == StatusCode::UNPROCESSABLE_ENTITY {
        let client_error_body = json!({
            "error": ClientError::INVALID_PARAMS
        });
        return (StatusCode::BAD_REQUEST, Json(client_error_body)).into_response();
    }

    // Log and modify error responses
    if let Some(service_error) = res.extensions().get::<Error>() {
        let (client_status, client_error) = service_error.client_status_and_error();
        let client_error_body = json!({
            "error": client_error,
        });

        // Log the server error
        tracing::error!(error = %service_error, "[ERROR OCCURRED]");

        return (client_status, Json(client_error_body)).into_response();
    }

    res
}
