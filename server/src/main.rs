use k6r::{Result, Server};

#[tokio::main]
async fn main() -> Result<()> {
    let bind = "127.0.0.1:8000";

    let server = Server::build(bind).await?;

    // Spawn a new asynchronous task using `tokio::spawn`
    // Creates a non-blocking task that runs the server instance in the background
    let _ = tokio::spawn(server.run()).await?;

    Ok(())
}
