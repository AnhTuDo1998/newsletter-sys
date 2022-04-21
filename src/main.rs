use actix_web::web::Json;
use newsletter_sys::configuration::get_configuration;
use newsletter_sys::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_log::LogTracer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Forward Log event to Trace
    LogTracer::init().expect("Failed to set logger");

    // Default Trace will be at INFO level
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("newsletter-sys".into(), std::io::stdout);
    let subscriber = Registry::default().with(env_filter).with(JsonStorageLayer).with(formatting_layer);
    
    // Set default subscriber globally
    set_global_default(subscriber).expect("Failed to set subscriber");
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
