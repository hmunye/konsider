use axum::routing::{get, post};
use axum::Router;

use crate::api::controllers::{
    api_change_password, api_create_user, api_get_all_users, api_get_user,
};
use crate::server::ServerState;

pub fn user_routes() -> Router<ServerState> {
    // All routes are under the `/api/v1/users` path
    Router::new()
        .route("/", get(api_get_all_users).post(api_create_user))
        .route("/:user_id", get(api_get_user))
        .route("/password", post(api_change_password))
}
