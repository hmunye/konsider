use k6r::log::{get_subscriber, init_subscriber};
use k6r::{get_config, Result, Server};

#[tokio::main]
async fn main() -> Result<()> {
    // Creates an hourly rotating file appender that writes to ./logs/k6r.YYYY-MM-DD-HH
    // (New log file to write to every hour)
    let file_appender = tracing_appender::rolling::hourly("./logs", "k6r");

    // This spawns a dedicated worker thread which is responsible for writing log lines to the provided writer
    let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);

    // Change 'file_writer' to 'std::io::stdout' to view logs in terminal instead
    let subscriber = get_subscriber("k6r".into(), "info".into(), file_writer);
    init_subscriber(subscriber);

    let config = get_config().expect("failed to read config file");

    let server = Server::build(config.clone()).await?;

    // Spawn a new asynchronous task using `tokio::spawn`
    // Creates a non-blocking task that runs the server instance in the background
    let _ = tokio::spawn(server.run()).await?;

    Ok(())
}
