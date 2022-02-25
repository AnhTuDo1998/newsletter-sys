use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server =
        HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check_handler)))
            .bind(("127.0.0.1", 8080))?
            .run();
    Ok(server)
}
