use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

// Type alias for Result
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    // -- server (unexpected errors)
    ServerError(std::sync::Arc<dyn std::error::Error + Send + Sync>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<tokio::task::JoinError> for Error {
    fn from(err: tokio::task::JoinError) -> Self {
        Error::ServerError(std::sync::Arc::new(err))
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
