use k6r::api::{log_cleanup_task, poll_and_update_token_cache, TokenCache};
use k6r::config::get_config;
use k6r::log::{get_subscriber, init_subscriber};
use k6r::Server;

#[tokio::main]
async fn main() -> k6r::Result<()> {
    // Creates an hourly rotating file appender that writes to logs/k6r.YYYY-MM-DD-HH
    // (New log file to write to every hour)
    let file_appender = tracing_appender::rolling::hourly("./logs", "k6r");

    // This spawns a dedicated worker thread which is responsible for writing log lines to the provided writer
    let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);

    // Change 'file_writer' to 'std::io::stdout' to view logs in terminal instead
    let subscriber = get_subscriber("k6r".into(), "info".into(), file_writer);
    init_subscriber(subscriber);

    let config = get_config().expect("failed to read config file");

    // Initialize cache to store valid tokens in-memory
    let token_cache = TokenCache::new();

    let (server, tcp_listener) = Server::build(config.clone(), token_cache.clone()).await?;

    let log_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("logs");
    let log_retention_days = config.log.retention_days;

    // Spawn three new asynchronous tasks using `tokio::spawn`
    let server_task = tokio::spawn(server.run(tcp_listener));
    let worker_task = tokio::spawn(poll_and_update_token_cache(token_cache, config.database));
    let log_cleanup_task = tokio::spawn(log_cleanup_task(log_dir, log_retention_days));

    tokio::select! {
        t = server_task => report_exit("SERVER", t),
        t = worker_task => report_exit("WORKER", t),
        t = log_cleanup_task => report_exit("LOG CLEANUP", Ok(t))
    }

    Ok(())
}

fn report_exit(
    task_name: &str,
    outcome: Result<Result<(), impl std::fmt::Debug + std::fmt::Display>, tokio::task::JoinError>,
) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed",
                task_name
            )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{}' task failed to complete",
                task_name
            )
        }
    }
}
