use actix_web::web::Form;
use actix_web::web::HttpResponse;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe_handler(form: Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
