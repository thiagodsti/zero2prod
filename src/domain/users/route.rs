use actix_web::{post, web, HttpResponse};
use sqlx::PgPool;

use crate::domain::users::dtos::new_user_dto::NewUserDto;
use crate::domain::users::entities::new_user::NewUser;
use crate::domain::users::repository::{UserRepositoryImpl};
use crate::domain::users::service::{UserService, UserServiceImpl};

#[tracing::instrument(name = "Adding a new user", skip(new_user_dto, pool))]
#[post("")]
pub async fn save_new_user(
    new_user_dto: web::Json<NewUserDto>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let new_user: NewUser = match new_user_dto.0.try_into() {
        Ok(user) => user,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let service = UserServiceImpl::new(pool);
    match service.save_user(&new_user).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
