use newsletter_sys::configuration::get_configuration;
use newsletter_sys::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
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
