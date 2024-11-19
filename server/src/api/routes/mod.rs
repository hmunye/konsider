mod auth_routes;
mod health_routes;
mod requester_routes;
mod software_request_routes;
mod software_routes;
mod user_routes;

pub use auth_routes::auth_routes;
pub use health_routes::health_routes;
pub use requester_routes::requester_routes;
pub use software_request_routes::software_request_routes;
pub use software_routes::software_routes;
pub use user_routes::user_routes;
