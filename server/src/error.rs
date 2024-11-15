use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

// Type alias for Result
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    // (unexpected errors)
    #[error(transparent)]
    ServerError(std::sync::Arc<anyhow::Error>),
}

impl From<tokio::task::JoinError> for Error {
    fn from(err: tokio::task::JoinError) -> Self {
        Error::ServerError(std::sync::Arc::new(err.into()))
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        // Just a placeholder response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the response
        response.extensions_mut().insert(self);

        response
    }
}
