use axum::routing::{delete, get};
use axum::Router;

use crate::api::controllers::{
    api_create_software_review, api_delete_software_review, api_get_all_software_reviews,
};
use crate::server::ServerState;

pub fn software_review_routes() -> Router<ServerState> {
    // All routes are under the `/api/v1/reviews` path
    Router::new()
        .route(
            "/",
            get(api_get_all_software_reviews).post(api_create_software_review),
        )
        .route("/:review_id", delete(api_delete_software_review))
}
