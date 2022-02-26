use std::net::TcpListener;

#[tokio::test]
async fn test_health_check_endpoint() {
    let binded_address = spawn_app();

    // Init client and get response
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check",binded_address))
        .send()
        .await
        .expect("Failed to execute the request.");

    // Test asserts
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

fn spawn_app() -> String {
    // Bind port and retrieve the random port allocated
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    // Server config
    let test_server = newsletter_sys::run(listener).expect("Failed to bind address!");
    // launch the server on the background
    let _ = tokio::spawn(test_server);

    // return address to the caller
    format!("http://127.0.0.1:{}", port)
}
