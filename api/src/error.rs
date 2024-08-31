// TODO: Look into anyhow and thiserror for better error handling

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

// ---------------------------------------------------------------------------------------------------------------
#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    EmailNotFoundError(String),

    #[error("{0}")]
    InvalidPasswordError(String),

    #[error("validation error occured while parsing user payload: {0}")]
    UserValidationError(String),

    #[error("{1}")]
    UnexpectedError(
        #[source] std::sync::Arc<dyn std::error::Error + Send + Sync>,
        String,
    ),

    #[error("no auth token provided")]
    NoAuthProvidedError,

    #[error("role is not vaild for the requested endpoint")]
    InvalidRoleError,

    #[error("")]
    IdempotencyError,
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    INVALID_CREDENTIALS,
    INVALID_PERMISSIONS,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}

// Used for 'main_response_mapper' middleware
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        // Just a placeholder response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the response
        response.extensions_mut().insert(self);

        response
    }
}

// Converting server errors to client errors
impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            Self::EmailNotFoundError(..) | Self::InvalidPasswordError(..) => {
                (StatusCode::UNAUTHORIZED, ClientError::INVALID_CREDENTIALS)
            }

            Self::NoAuthProvidedError => (StatusCode::UNAUTHORIZED, ClientError::NO_AUTH),

            Self::InvalidRoleError => (StatusCode::FORBIDDEN, ClientError::INVALID_PERMISSIONS),

            Self::UserValidationError(..) => (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS),

            // -- Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}
