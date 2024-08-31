use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use tower_sessions::Session;
use uuid::Uuid;

use crate::Error;

pub struct TypedSession(pub Session);

impl TypedSession {
    const USER_ID_KEY: &'static str = "user_id";

    pub async fn cycle(&self) -> crate::Result<()> {
        self.0.cycle_id().await.map_err(|err| {
            Error::UnexpectedError(
                std::sync::Arc::new(err),
                "Failed to cycle session id".into(),
            )
        })?;

        Ok(())
    }

    pub async fn insert_user_id(&self, user_id: Uuid) -> crate::Result<()> {
        self.0
            .insert(Self::USER_ID_KEY, user_id)
            .await
            .map_err(|err| {
                Error::UnexpectedError(
                    std::sync::Arc::new(err),
                    "Failed to insert user id into session store".into(),
                )
            })?;

        Ok(())
    }

    pub async fn get_user_id(&self) -> crate::Result<Option<Uuid>> {
        let user_id = self
            .0
            .get::<Uuid>(Self::USER_ID_KEY)
            .await
            .map_err(|_| Error::NoAuthProvidedError)?;

        Ok(user_id)
    }

    pub async fn log_out_user(self) -> crate::Result<()> {
        self.0.flush().await.map_err(|err| {
            Error::UnexpectedError(
                std::sync::Arc::new(err),
                "Failed to flush user's session".into(),
            )
        })?;

        Ok(())
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for TypedSession
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(req, state).await.map_err(|_| {
            Error::UnexpectedError(
                std::sync::Arc::new(Error::NoAuthProvidedError),
                "Failed to get session from request parts".into(),
            )
        })?;

        Ok(TypedSession(session))
    }
}
