use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::types::chrono::Utc;
use sqlx::types::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Subscriber {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Json<Subscriber>, pool: web::Data<PgPool>) -> HttpResponse {
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
// We use `get_ref` to get an immutable reference to the `PgConnection`
// wrapped by `web::Data`.
        .execute(pool.get_ref())
        .await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}