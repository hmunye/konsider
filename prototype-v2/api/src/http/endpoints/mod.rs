use axum::{
    routing::{get, post},
    Router,
};

mod create_post;
mod get_posts;
mod health_check;

pub use create_post::*;
pub use get_posts::*;
pub use health_check::*;

use crate::http::AppState;

pub fn api_router(state: AppState) -> Router {
    Router::new()
        .route("/v1/healthcheck", get(health_check))
        .route("/v1/posts", post(create_post))
        .route("/v1/posts", get(get_posts))
        .with_state(state.clone())
}
