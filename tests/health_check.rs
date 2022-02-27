use newsletter_sys::configuration::get_configuration;
use sqlx::{Connection, PgConnection};
use std::net::TcpListener;

fn spawn_app() -> String {
    // Bind port and retrieve the random port allocated
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    // Server config
    let test_server = newsletter_sys::startup::run(listener).expect("Failed to bind address!");
    // launch the server on the background
    let _ = tokio::spawn(test_server);

    // return address to the caller
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn test_health_check_endpoint() {
    let binded_address = spawn_app();

    // Init client and get response
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", binded_address))
        .send()
        .await
        .expect("Failed to execute the request.");

    // Test asserts
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

#[tokio::test]
async fn test_subscribe_valid_form_data() {
    /*
    This test checks if the response status code is 200
     */
    let address = spawn_app();
    let configs = get_configuration().expect("Failed to read configs!");
    let connection_string = configs.database.connection_string();

    // DB connection
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");

    let client = reqwest::Client::new();

    // simulate post form body
    let body = "name=boom%20do&email=tommyboom1998%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscriptions");

    assert_eq!(saved.email, "tommyboom1998@gmail.com");
    assert_eq!(saved.name, "boom do");
}

#[tokio::test]
async fn test_subscribe_invalid_form_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    // parameterized testing for fail case
    let test_cases = vec![
        ("name=boom%20do", "missing valid email!"),
        ("tommyboom1998%40gmail.com", "missing a name!"),
        ("", "missing both name and email!"),
    ];

    for test_case in test_cases {
        let body = test_case.0;
        let error_msg = test_case.1;

        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_msg
        );
    }
}
