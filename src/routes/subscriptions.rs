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
    log::info!(
        "Adding '{}' '{}' as a new subscriber.", form.email, form.name
    );

    log::info!("Saving new subscriber details in the database");

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
            log::info!("New subscriber details have been saved");
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            log::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
