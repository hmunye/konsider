use k6r::{get_config, server::Server};

// Type alias for Result
pub type Result<T> = std::result::Result<T, Error>;

// Using `Box<dyn std::error::Error>` for flexibility in error handling
pub type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
pub struct TestServer {
    pub addr: String,
    pub client: reqwest::Client,
}

// Provides method for sending GET HTTP requests to a specified URL
impl TestServer {
    pub async fn get_request(&self, url: &String) -> Result<reqwest::Response> {
        Ok(self
            .client
            .get(url)
            .send()
            .await
            .map_err(|err| format!("failed to execute request: cause {err}"))?)
    }
}

pub async fn spawn_server() -> Result<TestServer> {
    let config = {
        let mut config = get_config().expect("failed to read config");

        // Using port '0' will trigger the OS to scan for an available port.
        // This allows the server to continue running on port 8000 while each
        // test is executed using a different port, avoiding port conflicts
        config.server.port = 0;

        config
    };

    let server = Server::build(config.clone()).await?;

    let port = server.port();

    // Spawn a new asynchronous task using `tokio::spawn`.
    // Creates a non-blocking task that runs the server instance in the background.
    // The server's `run` method is awaited within this task, allowing it to
    // handle incoming requests while the main thread can continue executing
    tokio::spawn(server.run());

    let client = reqwest::Client::builder().build()?;

    let test_server = TestServer {
        addr: format!("http://127.0.0.1:{}", port),
        client,
    };

    Ok(test_server)
}
