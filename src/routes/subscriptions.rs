use actix_web::web;
use actix_web::web::Form;
use actix_web::web::HttpResponse;
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe_handler(form: Form<FormData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    // generate random request_id for use in logging
    let request_id = Uuid::new_v4();
    log::info!(
        "request_id {} - Adding '{}' '{}' as a new subscriber.", request_id, form.email, form.name
    );

    log::info!("request_id {} - Saving new subscriber details in the database", request_id);

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // get_ref is to get immutable ref of PgConnection wrapped in web::Data ptr
    .execute(db_pool.as_ref())
    .await
    {
        Ok(_) => {
            log::info!("request_id {} - New subscriber details have been saved", request_id);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            log::error!("request_id {} - Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
