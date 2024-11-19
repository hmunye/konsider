use axum::routing::get;
use axum::Router;

use crate::api::controllers::api_get_all_software;
use crate::server::ServerState;

pub fn software_routes() -> Router<ServerState> {
    // All routes are under the `/api/v1/software` path
    Router::new().route("/", get(api_get_all_software))
}
