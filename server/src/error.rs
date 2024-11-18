use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

use crate::api::{JsonError, PathError};

// Type alias for Result
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, thiserror::Error)]
pub enum Error {
    // -- auth
    #[error("no account found associated with the provided email")]
    AuthEmailNotFoundError,
    #[error("provided password is invalid")]
    AuthInvalidPasswordError,
    #[error("the provided token for the request is invalid")]
    AuthInvalidTokenError,
    #[error("request is missing a valid token")]
    AuthMissingTokenError,
    #[error("user role is invaild for the requested endpoint")]
    AuthInvalidRoleError,

    // -- validation
    #[error("validation error occured while parsing {0}")]
    ValidationError(String),
    #[error("validation error occured while parsing query parameters: {0}")]
    QueryParamValidationError(String),

    // -- database
    #[error("database record could not be found")]
    PgNotFoundError,
    #[error("database record already exists")]
    PgRecordExists,
    #[error("database key violation occured")]
    PgKeyViolation,

    // -- other
    #[error("error occured parsing JSON payload from request: {0}")]
    PayloadExtractorError(JsonError),
    #[error("error occured parsing path parameters from request: {0}")]
    PathExtractorError(PathError),

    // (unexpected errors)
    #[error(transparent)]
    ServerError(std::sync::Arc<Box<dyn std::error::Error + Send + Sync>>),
}

fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    write!(f, "{}", e)?;

    let current = e.source();

    if let Some(cause) = current {
        write!(f, " CAUSE: {}", cause)?;
    }

    Ok(())
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(err: tokio::task::JoinError) -> Self {
        Error::ServerError(std::sync::Arc::new(err.into()))
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
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

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, String) {
        match self {
            Self::AuthEmailNotFoundError | Self::AuthInvalidPasswordError => (
                StatusCode::UNAUTHORIZED,
                ClientError::InvalidCredentials.to_string(),
            ),

            Self::AuthInvalidTokenError => (
                StatusCode::UNAUTHORIZED,
                ClientError::InvalidToken.to_string(),
            ),

            Self::AuthMissingTokenError => (
                StatusCode::UNAUTHORIZED,
                ClientError::MissingToken.to_string(),
            ),

            Self::AuthInvalidRoleError => {
                (StatusCode::FORBIDDEN, ClientError::InvalidRole.to_string())
            }

            Self::QueryParamValidationError(..) | Self::PathExtractorError(..) => (
                StatusCode::BAD_REQUEST,
                ClientError::InvalidParams.to_string(),
            ),

            Self::PayloadExtractorError(..) | Self::ValidationError(..) => (
                StatusCode::BAD_REQUEST,
                ClientError::InvalidPayload.to_string(),
            ),

            Self::PgKeyViolation => (StatusCode::CONFLICT, ClientError::Conflict.to_string()),

            Self::PgRecordExists => (StatusCode::CONFLICT, ClientError::RecordExists.to_string()),

            Self::PgNotFoundError => (StatusCode::NOT_FOUND, ClientError::NotFound.to_string()),

            // -- Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::ServiceError.to_string(),
            ),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum ClientError {
    InvalidCredentials,
    InvalidPayload,
    InvalidParams,
    InvalidRole,
    InvalidToken,
    MissingToken,
    NotFound,
    RecordExists,
    Conflict,
    ServiceError,
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_message = match self {
            ClientError::InvalidCredentials => "The provided credentials are invalid",
            ClientError::InvalidPayload => {
                "The submitted payload is invalid or does not conform to the expected format"
            }
            ClientError::InvalidRole => {
                "You do not have sufficient permissions to perform this action"
            }
            ClientError::InvalidParams => "The supplied parameters are invalid for this request",
            ClientError::InvalidToken => "The provided token for the request is invalid",
            ClientError::MissingToken => "The request is missing a valid token",
            ClientError::NotFound => "The requested resource could not be found",
            ClientError::Conflict => "The request could not be completed due to a conflict",
            ClientError::RecordExists => "A record with the specified details already exists",
            _ => "An internal server error has occurred. Please try again later",
        };

        write!(f, "{}", error_message)
    }
}
