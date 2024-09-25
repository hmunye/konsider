use axum::routing::{get, post};
use axum::Router;

use crate::server::AppState;
use crate::web::auth::{api_check_auth, api_login, api_logout};
use crate::web::reject_unauthorized_users;

// ---------------------------------------------------------------------------------------------------------------
pub fn auth_routes(state: AppState) -> Router {
    Router::new()
        .route("/login", post(api_login).with_state(state.clone()))
        .route("/logout", post(api_logout))
        .route(
            "/check",
            get(api_check_auth).layer(axum::middleware::from_fn(reject_unauthorized_users)),
        )
}
