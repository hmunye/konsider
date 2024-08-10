mod common;

#[tokio::test]
async fn health_check_test() {
    let client = reqwest::Client::new();
    let server = common::spawn_server().await;
    let url = format!("{}/v1/healthcheck", server.addr);

    let response = client
        .get(&url)
        .send()
        .await
        .expect("Failed to process request");

    assert!(response.status().is_success());
    assert_eq!(Some(14), response.content_length());
}
