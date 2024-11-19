use axum::routing::{delete, get};
use axum::Router;

use crate::api::controllers::{api_create_software, api_delete_software, api_get_all_software};
use crate::server::ServerState;

pub fn software_routes() -> Router<ServerState> {
    // All routes are under the `/api/v1/software` path
    Router::new()
        .route("/", get(api_get_all_software).post(api_create_software))
        .route("/:software_id", delete(api_delete_software))
}
