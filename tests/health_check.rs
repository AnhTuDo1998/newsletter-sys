#[tokio::test]
async fn test_health_check_endpoint() {
    spawn_app();

    // Init client and get response
    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("Failed to execute the request.");

    // Test asserts
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

fn spawn_app() {
    let test_server = newsletter_sys::run().expect("Failed to bind address!");
    // launch the server on the background
    let _ = tokio::spawn(test_server);
}
