// TODO: Look into anyhow and thiserror for better error handling

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------------------------------------------
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ServerError {
    LoginError(String),
    LogoutError(String),

    FetchUserError(String),
    UserValidationError(String),
    InsertUserError(String),

    DatabaseError(String),

    UnexpectedError(String),

    NoAuthProvided,

    InvalidRole,
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

impl std::fmt::Display for ServerError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ServerError::LoginError(msg) => write!(fmt, "login failed: {}", msg),
            ServerError::LogoutError(msg) => write!(fmt, "logout failed: {}", msg),
            ServerError::UserValidationError(msg) => write!(
                fmt,
                "user validation error occured while parsing payload: {}",
                msg
            ),
            ServerError::InsertUserError(msg) => write!(
                fmt,
                "database error occured while trying to insert user: {}",
                msg
            ),
            ServerError::FetchUserError(msg) => {
                write!(
                    fmt,
                    "database error occured while attempting to fetch user: {}",
                    msg
                )
            }
            ServerError::DatabaseError(msg) => write!(fmt, "database error occured: {}", msg),
            ServerError::UnexpectedError(msg) => write!(fmt, "unexpected error occured: {}", msg),
            ServerError::NoAuthProvided => write!(fmt, "no auth token provided"),
            ServerError::InvalidRole => write!(fmt, "role is not vaild for the requested endpoint"),
        }
    }
}

impl std::error::Error for ServerError {}

// Used for 'main_response_mapper' middleware
impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        // Just a placeholder response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the response
        response.extensions_mut().insert(self);

        response
    }
}

// Converting server errors to client errors
impl ServerError {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            Self::LoginError(..) | Self::FetchUserError(..) => {
                (StatusCode::UNAUTHORIZED, ClientError::INVALID_CREDENTIALS)
            }

            Self::NoAuthProvided => (StatusCode::UNAUTHORIZED, ClientError::NO_AUTH),

            Self::InvalidRole => (StatusCode::FORBIDDEN, ClientError::INVALID_PERMISSIONS),

            Self::UserValidationError(..) => (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS),

            // -- Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}
