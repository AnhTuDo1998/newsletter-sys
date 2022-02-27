use newsletter_sys::configuration::get_configuration;
use sqlx::PgPool;
use std::net::TcpListener;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    // Bind port and retrieve the random port allocated
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    // Once got random address port...
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    // Server config
    let configs = get_configuration().expect("Failed to read configuration.");
    let db_pool = PgPool::connect(&configs.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let test_server =
        newsletter_sys::startup::run(listener, db_pool.clone()).expect("Failed to bind address!");
    // launch the server on the background
    let _ = tokio::spawn(test_server);

    // Return a testapp that store configs and connection db states.
    TestApp { address, db_pool }
}

#[tokio::test]
async fn test_health_check_endpoint() {
    let app = spawn_app().await;

    // Init client and get response
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", &app.address))
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
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    // simulate post form body
    let body = "name=boom%20do&email=tommyboom1998%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscriptions");

    assert_eq!(saved.email, "tommyboom1998@gmail.com");
    assert_eq!(saved.name, "boom do");
}

#[tokio::test]
async fn test_subscribe_invalid_form_data() {
    let app = spawn_app().await;
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
            .post(&format!("{}/subscriptions", &app.address))
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
