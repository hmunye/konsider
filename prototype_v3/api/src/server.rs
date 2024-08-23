// TODO: Add middleware that limits the size of request bodies by default
// Reject any input over 4 GiB,
// or any input that could _encode_ to a string longer than 4 GiB

use axum::http::{Request, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, IntoMakeService};
use axum::serve::Serve;
use axum::{Json, Router};
use secrecy::ExposeSecret;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tower_http::classify::StatusInRangeAsFailures;
use tower_http::trace::TraceLayer;
use tracing::Level;
use uuid::Uuid;

use crate::error::ClientError;
use crate::web::{admin_routes, auth_routes, health_check};
use crate::{Config, Error};

type Server = Serve<IntoMakeService<Router>, Router>;

pub struct Application {
    port: u16,
    host: String,
    server: Server,
}

impl Application {
    pub async fn build(config: Config) -> Result<Self, std::io::Error> {
        let db_pool = get_db_pool(&config);

        let addr = format!("{}:{}", config.server_host, config.server_port);

        let tcp_listener = tokio::net::TcpListener::bind(&addr)
            .await
            .expect("Failed to bind to address");

        let host = config.server_host;

        let port = tcp_listener.local_addr().unwrap().port();

        let server = serve(tcp_listener, db_pool)?;

        Ok(Self { port, host, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn host(&self) -> String {
        self.host.clone()
    }

    pub async fn run_server(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_db_pool(config: &Config) -> PgPool {
    PgPoolOptions::new()
        // The amount of time the pool will wait to aquire a connection
        .acquire_timeout(std::time::Duration::from_secs(10))
        // The amount of time a connection can stay idle in the pool before it is closed
        .idle_timeout(std::time::Duration::from_secs(60))
        // Won't connect until a query is made
        .connect_lazy(config.connection_string().expose_secret())
        .expect("Failed to create connection pool")
}

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}

pub fn serve(
    tcp_listener: tokio::net::TcpListener,
    db_pool: PgPool,
) -> Result<Server, std::io::Error> {
    let state = AppState { db_pool };

    let routes_all = Router::new()
        .route("/health-check", get(health_check))
        .nest("/auth", auth_routes(state.clone()))
        .nest("/admin", admin_routes(state.clone()))
        .layer(axum::middleware::map_response(main_response_mapper))
        .layer(TraceLayer::new(
            // By default the trace layer only classifies 5xx errors as failures
            StatusInRangeAsFailures::new(400..=599).into_make_classifier(),
        ))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let request_id = Uuid::new_v4().to_string();

                // Will be included with every request
                tracing::span!(
                    Level::DEBUG,
                    "request",
                    %request_id,
                    method = ?request.method(),
                    uri = %request.uri(),
                    version = ?request.version(),
                )
            }),
        );

    Ok(axum::serve(tcp_listener, routes_all.into_make_service()))
}

// Modify responses before they are sent to the client
async fn main_response_mapper(res: Response) -> Response {
    let status_code = res.status();

    // Handle any 422 status codes with custom response, minimizing information disclosure
    // Ex. Request is missing a field for specific struct
    if status_code == StatusCode::UNPROCESSABLE_ENTITY {
        let client_error_body = json!({
            "error": ClientError::INVALID_PARAMS
        });

        // Build new response
        return (StatusCode::BAD_REQUEST, Json(client_error_body)).into_response();
    }

    // Get the response error
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    // If there is a client error, build the new reponse
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": client_error,
            });

            // Build new response
            (*status_code, Json(client_error_body)).into_response()
        });

    println!();
    error_response.unwrap_or(res)
}
