use axum::routing::get;
use axum::Router;

use crate::api::controllers::{api_create_requester, api_get_all_requesters};
use crate::server::ServerState;

pub fn requester_routes() -> Router<ServerState> {
    // All routes are under the `/api/v1/requesters` path
    Router::new().route("/", get(api_get_all_requesters).post(api_create_requester))
}
