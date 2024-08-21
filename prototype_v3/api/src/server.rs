// TODO: Add middleware that limits the size of request bodies by default
// Reject any input over 4 GiB,
// or any input that could _encode_ to a string longer than 4 GiB

use axum::http::Request;
use axum::routing::{get, IntoMakeService};
use axum::serve::Serve;
use axum::Router;
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use tracing::Level;

use crate::web::{admin_routes, auth_routes, health_check};
use crate::Config;

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
        tracing::info!(
            "->> {:<12} - {}",
            "LISTENING",
            format!("{}:{}", self.host, self.port)
        );

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
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let request_id = uuid::Uuid::new_v4().to_string();

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
