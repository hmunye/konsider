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

    #[error("no session token provided for request")]
    NoAuthProvidedError,

    #[error("role is not vaild for the requested endpoint")]
    InvalidRoleError,

    #[error("user could not be found")]
    UserNotFoundError,

    #[error("email is already in use")]
    EmailInUseError,

    #[error("No details provided to update user")]
    NoUpdatesProvidedError,

    #[error("{1}")]
    UnexpectedError(
        #[source] std::sync::Arc<dyn std::error::Error + Send + Sync>,
        String,
    ),
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    INVALID_CREDENTIALS,
    NOT_FOUND,
    INVALID_PERMISSIONS,
    NO_AUTH,
    CONFLICT,
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

            Self::UserNotFoundError => (StatusCode::NOT_FOUND, ClientError::NOT_FOUND),

            Self::EmailInUseError => (StatusCode::CONFLICT, ClientError::CONFLICT),

            Self::InvalidRoleError => (StatusCode::FORBIDDEN, ClientError::INVALID_PERMISSIONS),

            Self::UserValidationError(..) | Self::NoUpdatesProvidedError => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }

            // -- Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}
