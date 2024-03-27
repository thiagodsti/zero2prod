use actix_web::{HttpResponse, post, web};
use actix_web::web::{Json};
use sqlx::PgPool;

use crate::domain::users::dtos::new_user_dto::NewUserDto;
use crate::domain::users::entities::new_user::NewUser;
use crate::domain::users::repository::{UserRepository, UserRepositoryImpl};
use crate::domain::users::service::{UserService, UserServiceImpl};

//
// pub trait UserRoute<T,R> where T: UserService<R>,
// R: UserRepository + 'static{
//
//     fn new(service: T) -> Self;
//     async fn save_new_user(
//         &self, new_user_dto: Json<NewUserDto>,
//     ) -> HttpResponse;
// }
//
// pub struct UserRouteImpl<T,R> where
//     T: UserService<R>,
//     R: UserRepository + 'static {
//     service: T,
//     _marker: std::marker::PhantomData<R> // necessary because R is not being used
// }
//
//
// impl<T,R> UserRoute<T,R> for UserRouteImpl<T,R>
//     where T: UserService<R>,
//         R: UserRepository + 'static{
//     fn new(service: T) -> Self {
//         Self {
//             service,
//             _marker: Default::default()
//         }
//     }
//
//     #[tracing::instrument(name = "Adding a new user", skip(new_user_dto, self))]
//     async fn save_new_user(
//         &self,
//         new_user_dto: Json<NewUserDto>,
//     ) -> HttpResponse {
//         let new_user: NewUser = match new_user_dto.0.try_into() {
//             Ok(user) => user,
//             Err(_) => return HttpResponse::BadRequest().finish(),
//         };
//
//         //let service = UserServiceImpl::new(UserRepositoryImpl::new(pool));
//         match self.service.save_user(&new_user).await {
//             Ok(_) => HttpResponse::Created().finish(),
//             Err(_) => HttpResponse::InternalServerError().finish(),
//         }
//     }
// }

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

    let service = UserServiceImpl::new(UserRepositoryImpl::new(pool));
    match service.save_user(&new_user).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
