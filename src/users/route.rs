use actix_web::{HttpResponse, web};
use sqlx::PgPool;

use crate::users::dtos::NewUserDto;
use crate::users::entities::new_user::NewUser;

#[tracing::instrument(
name = "Adding a new user",
skip(new_user_dto, pool)
)]
pub async fn subscribe(new_user_dto: web::Json<NewUserDto>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_user: NewUser = match new_user_dto.0.try_into() {
        Ok(new_user) => new_user,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    HttpResponse::Ok().finish()
/*    match insert_subscriber(&pool, &new_subscriber).await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }*/
}