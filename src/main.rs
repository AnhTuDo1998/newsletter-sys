use newsletter_sys::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed in binding address/port!");
    let port = listener.local_addr().unwrap().port();
    println!("Allocated port: {}", port);
    run(listener)?.await
}
