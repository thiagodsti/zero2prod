use actix_web::{HttpResponse, web};
use sqlx::PgPool;

use crate::domain::users::dtos::NewUserDto;
use crate::domain::users::entities::new_user::NewUser;
use crate::domain::users::service::save_user;

#[tracing::instrument(name = "Adding a new user", skip(form, pool))]
pub async fn save_new_user(form: web::Json<NewUserDto>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_user: NewUser = match form.0.try_into() {
        Ok(user) => user,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    match save_user(&new_user, pool).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
