use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use tower_sessions_redis_store::fred::error::RedisError;

pub type Result<T> = std::result::Result<T, Error>;

// ---------------------------------------------------------------------------------------------------------------
#[derive(Clone, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    EmailNotFoundError(String),

    #[error("{0}")]
    InvalidPasswordError(String),

    #[error("Validation error occured while parsing user payload: {0}")]
    UserValidationError(String),

    #[error("No session token provided for request")]
    NoAuthProvidedError,

    #[error("Role is not vaild for the requested endpoint")]
    InvalidRoleError,

    #[error("Database record could not be found")]
    NotFoundError,

    #[error("Error updating the database record due to an edit conflict")]
    EditConflictError,

    #[error("Email is already in use")]
    EmailInUseError,

    #[error("No details provided to update user")]
    NoUpdatesProvidedError,

    #[error("Error occured validating idempotency key")]
    IdempotencyKeyError,

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

fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;

    let mut current = e.source();

    while let Some(cause) = current {
        writeln!(f, "CAUSED BY:\n\t{}", cause)?;
        current = cause.source();
    }

    Ok(())
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl From<RedisError> for Error {
    fn from(err: RedisError) -> Self {
        Error::UnexpectedError(std::sync::Arc::new(err), "Redis error occured".into())
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::UnexpectedError(std::sync::Arc::new(err), "PostgreSQL error occured".into())
    }
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

            Self::NotFoundError => (StatusCode::NOT_FOUND, ClientError::NOT_FOUND),

            Self::EmailInUseError | Self::EditConflictError => {
                (StatusCode::CONFLICT, ClientError::CONFLICT)
            }

            Self::InvalidRoleError => (StatusCode::FORBIDDEN, ClientError::INVALID_PERMISSIONS),

            Self::UserValidationError(..)
            | Self::NoUpdatesProvidedError
            | Self::IdempotencyKeyError => (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS),

            // -- Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}
