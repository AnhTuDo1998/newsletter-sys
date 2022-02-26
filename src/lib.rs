use actix_web::dev::Server;
use actix_web::web::Form;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::net::TcpListener;

async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe_handler(form: Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check_handler))
            .route("/subscriptions", web::post().to(subscribe_handler))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
