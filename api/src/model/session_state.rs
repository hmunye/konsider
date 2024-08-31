use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use tower_sessions::Session;
use uuid::Uuid;

use crate::ServerError;

pub struct TypedSession(pub Session);

impl TypedSession {
    const USER_ID_KEY: &'static str = "user_id";

    pub async fn cycle(&self) -> Result<(), ServerError> {
        self.0
            .cycle_id()
            .await
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

        Ok(())
    }

    pub async fn insert_user_id(&self, user_id: Uuid) -> Result<(), ServerError> {
        self.0
            .insert(Self::USER_ID_KEY, user_id)
            .await
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

        Ok(())
    }

    pub async fn get_user_id(&self) -> Result<Option<Uuid>, ServerError> {
        let user_id = self
            .0
            .get::<Uuid>(Self::USER_ID_KEY)
            .await
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

        Ok(user_id)
    }

    pub async fn log_out_user(self) -> Result<(), ServerError> {
        self.0
            .flush()
            .await
            .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

        Ok(())
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for TypedSession
where
    S: Send + Sync,
{
    type Rejection = ServerError;

    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(req, state)
            .await
            .map_err(|err| ServerError::UnexpectedError(err.1.to_string()))?;

        Ok(TypedSession(session))
    }
}
