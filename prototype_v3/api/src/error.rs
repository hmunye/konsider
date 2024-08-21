use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    LoginFail,
    UserValidationError,
    InsertUserFail(String),
    FetchUserFailEmailNotFound(String),
    DatabaseError(String),
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
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Error::FetchUserFailEmailNotFound(err.to_string()),
            sqlx::Error::Database(err) => {
                if err.is_unique_violation() {
                    Error::InsertUserFail(err.to_string())
                } else {
                    Error::UserValidationError
                }
            }
            _ => Error::DatabaseError(err.to_string()),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the reponse.
        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Self::LoginFail | Self::FetchUserFailEmailNotFound(_) => {
                (StatusCode::UNAUTHORIZED, ClientError::INVALID_CREDENTIALS)
            }

            Self::UserValidationError => (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS),

            // -- Fallback.
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}
