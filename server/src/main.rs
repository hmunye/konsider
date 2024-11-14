use k6r::{get_config, Result, Server};

#[tokio::main]
async fn main() -> Result<()> {
    let config = get_config().expect("failed to read config file");

    let server = Server::build(config.clone()).await?;

    // Spawn a new asynchronous task using `tokio::spawn`
    // Creates a non-blocking task that runs the server instance in the background
    let _ = tokio::spawn(server.run()).await?;

    Ok(())
}
