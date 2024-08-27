//use axum::async_trait;
//use axum::extract::FromRequestParts;
//use axum::http::header;
//use axum::http::request::Parts;
//use tower_sessions::Session;
//
//use crate::Error;
//
//#[allow(dead_code)]
//pub struct AuthSession(pub Session);
//
//#[async_trait]
//impl<S> FromRequestParts<S> for AuthSession
//where
//    S: Send + Sync,
//{
//    type Rejection = Error;
//
//    #[tracing::instrument(name = "auth middleware", skip(parts, _state))]
//    // Checks for session, then vaildates session, on every request when Auth is a middleware extractor
//    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
//        // Get AUTHORIZATION header, split, and grab token after "Bearer"
//        // Ex. Authorization: Bearer <token>
//        // The header is not part of the value converted to str
//        //        let access_token = parts
//        //            .headers
//        //            .get(header::AUTHORIZATION)
//        //            .and_then(|value| value.to_str().ok())
//        //            .and_then(|str| str.split(" ").nth(1));
//
//        // Get COOKIE header, split, and grab token after the name of the cookie
//        // Ex. Cookie: <name>=<jwt_token>
//        // The header is not part of the value converted to str
//        let session_id = parts
//            .headers
//            .get(header::COOKIE)
//            .and_then(|value| value.to_str().ok())
//            .and_then(|str| str.split("=").nth(1));
//
//        match session_id {
//            Some(id) =>
//
//            match user_id {
//                Ok(user_id) => Ok(AuthSession(user_id)),
//
//                Err(_) => Err(Error::NoAuthTokenProvided),
//            },
//
//            None => Err(Error::NoAuthTokenProvided),
//        }
//    }
//}
