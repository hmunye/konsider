use crate::common::{spawn_server, Result};

#[tokio::test]
async fn health_check_works() -> Result<()> {
    let server = spawn_server().await?;
    let url = format!("{}/api/v1/health", server.addr);

    let response = server.get_request(&url, None).await?;
    assert_eq!(204, response.status().as_u16());

    Ok(())
}
