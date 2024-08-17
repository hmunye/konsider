use api::telemetry::{get_subscriber, init_subscriber};
use api::web::server;
use api::Config;
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    let subscriber = get_subscriber("konsider_api".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    dotenvy::dotenv().ok();

    let config = Config::default();

    let db_pool = PgPool::connect(&config.connection_string()).await.unwrap();

    let tcp_listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.server_host, config.server_port))
            .await
            .unwrap();

    tracing::info!(
        "->> {:<12} - {}",
        "LISTENING",
        tcp_listener.local_addr().unwrap()
    );

    server::serve(tcp_listener, db_pool).await.unwrap()
}
