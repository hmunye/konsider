// TODO: Look into anyhow and thiserror

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

// ---------------------------------------------------------------------------------------------------------------
#[derive(Clone, Debug, Serialize)]
pub enum Error {
    LoginError(String),

    FetchUserError(String),
    UserValidationError(String),
    InsertUserError(String),

    DatabaseError(String),

    UnexpectedError(String),

    NoAuthTokenProvided,
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    INVALID_CREDENTIALS,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::LoginError(msg) => write!(fmt, "login failed: {}", msg),
            Error::UserValidationError(msg) => write!(
                fmt,
                "user validation error occured while parsing payload: {}",
                msg
            ),
            Error::InsertUserError(msg) => write!(
                fmt,
                "database error occured while trying to insert user: {}",
                msg
            ),
            Error::FetchUserError(msg) => {
                write!(
                    fmt,
                    "database error occured while attempting to fetch user: {}",
                    msg
                )
            }
            Error::DatabaseError(msg) => write!(fmt, "database error occured: {}", msg),
            Error::UnexpectedError(msg) => write!(fmt, "unexpected error occured: {}", msg),
            Error::NoAuthTokenProvided => write!(fmt, "no auth token provided"),
        }
    }
}

impl std::error::Error for Error {}

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
            Self::LoginError(..) | Self::FetchUserError(..) => {
                (StatusCode::UNAUTHORIZED, ClientError::INVALID_CREDENTIALS)
            }

            Self::NoAuthTokenProvided => (StatusCode::UNAUTHORIZED, ClientError::NO_AUTH),

            Self::UserValidationError(..) => (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS),

            // -- Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}
