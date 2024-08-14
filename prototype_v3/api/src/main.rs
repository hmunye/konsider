use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "status: ok")
}

#[tokio::main]
async fn main() {
    let routes = Router::new().route("/health-check", get(health_check));

    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:1234")
        .await
        .unwrap();

    println!("->> {:<12} - {}", "LISTENING", tcp_listener.local_addr().unwrap());

    axum::serve(tcp_listener, routes.into_make_service())
        .await
        .unwrap()
}
