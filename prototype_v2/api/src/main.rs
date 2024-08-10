use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Server Running").into_response()
}

#[tokio::main]
async fn main() {
    let routes = Router::new().route("/v1/healthcheck", get(health_check));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .expect("Failed to bind to address");

    println!("->> LISTENING on {}", listener.local_addr().unwrap());

    axum::serve(listener, routes.into_make_service())
        .await
        .expect("Failed to start server")
}
