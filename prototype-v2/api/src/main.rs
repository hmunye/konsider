use clap::Parser;
use konsider_api::{config::Config, http};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let config = Config::parse();

    let db = PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    let addr = format!("{}:{}", config.server_host, config.server_port);

    println!(
        "Server started on {}:{}",
        config.server_host, config.server_port
    );

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    http::serve(listener, db)?.await.unwrap();

    Ok(())
}
