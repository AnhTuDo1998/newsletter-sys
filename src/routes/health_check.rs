use actix_web::web::HttpResponse;

pub async fn health_check_handler() -> HttpResponse {
    HttpResponse::Ok().finish()
}
