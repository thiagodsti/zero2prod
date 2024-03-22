use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::users::dtos::NewUserDto;
use crate::users::entities::new_user::NewUser;
use crate::users::repository::insert_user;

#[tracing::instrument(name = "Adding a new user", skip(form, pool))]
pub async fn save_new_user(form: web::Json<NewUserDto>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_user: NewUser = match form.0.try_into() {
        Ok(user) => user,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    match insert_user(&pool, &new_user).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
