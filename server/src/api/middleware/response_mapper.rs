use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

use crate::Error;

// Modify error responses before they are sent to the client
pub async fn main_response_mapper(response: Response) -> Response {
    if let Some(service_error) = response.extensions().get::<Error>() {
        let (client_status, client_error) = service_error.client_status_and_error();
        let client_error_body = json!({
            "error": client_error,
        });

        tracing::error!(error = ?service_error, "[ERROR]");

        return (client_status, Json(client_error_body)).into_response();
    }

    response
}
