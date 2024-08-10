use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, IntoMakeService};
use axum::serve::Serve;
use axum::Router;

type Server = Serve<IntoMakeService<Router>, Router>;

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Server Running").into_response()
}

pub fn serve(listener: tokio::net::TcpListener) -> Result<Server, std::io::Error> {
    let routes = Router::new().route("/v1/healthcheck", get(health_check));

    let server = axum::serve(listener, routes.into_make_service());

    Ok(server)
}
