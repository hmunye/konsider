use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    let server = Router::new().route("/", get(api_health_check));

    let bind = "127.0.0.1:8080";

    let tcp_listener = tokio::net::TcpListener::bind(&bind).await.unwrap();

    println!(">> LISTENING ON {bind}");

    axum::serve(tcp_listener, server.into_make_service())
        .await
        .unwrap();
}

async fn api_health_check() -> StatusCode {
    StatusCode::NO_CONTENT
}
