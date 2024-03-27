use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::PgPool;

use crate::domain::users::dtos::new_user_dto::NewUserDto;
use crate::domain::users::entities::new_user::NewUser;
use crate::domain::users::repository::UserRepositoryImpl;
use crate::domain::users::service::{UserService, UserServiceImpl};

#[tracing::instrument(name = "Adding a new user", skip(new_user_dto, pool))]
pub async fn save_new_user(
    State(pool): State<PgPool>,
    Json(new_user_dto): Json<NewUserDto>,
) -> StatusCode {

    let new_user: NewUser = match new_user_dto.try_into() {
        Ok(user) => user,
        Err(_) => return StatusCode::BAD_REQUEST,
    };

   let service = UserServiceImpl::new(UserRepositoryImpl::new(pool));
    match service.save_user(&new_user).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
