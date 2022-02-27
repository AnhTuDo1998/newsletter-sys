use newsletter_sys::configuration::get_configuration;
use newsletter_sys::startup::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Read configuration
    let configs = get_configuration().expect("Failed to read configs!");
    let address = format!("127.0.0.1:{}", configs.application_port);
    let listener = TcpListener::bind(address).expect("Failed in binding address/port!");
    run(listener)?.await
}
