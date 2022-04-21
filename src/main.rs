use actix_web::web::Json;
use newsletter_sys::configuration::get_configuration;
use newsletter_sys::startup::run;
use newsletter_sys::telemetry::{get_subscriber, init_subscriber};
use sqlx::PgPool;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // For telemetry of application (tracing)
    let subscriber = get_subscriber("newsletter-sys".into(), "info".into());
    init_subscriber(subscriber);

    // Read configuration
    let configs = get_configuration().expect("Failed to read configs!");
    // DB connection
    let connection_pool = PgPool::connect(&configs.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configs.application_port);
    let listener = TcpListener::bind(address).expect("Failed in binding address/port!");
    run(listener, connection_pool)?.await
}
