use axum::routing::{delete, get, post};
use axum::Router;

use crate::api::controllers::{api_check_token, api_login, api_logout, api_revoke_user_token};
use crate::server::ServerState;

pub fn auth_routes() -> Router<ServerState> {
    // All routes are under the `/api/v1/auth` path
    Router::new()
        .route("/login", post(api_login))
        .route("/logout", post(api_logout))
        .route("/check", get(api_check_token))
        .route("/revoke/:user_id", delete(api_revoke_user_token))
}
