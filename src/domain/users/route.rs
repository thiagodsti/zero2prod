use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::{Error, PgPool};

use crate::domain::users::dtos::new_user_dto::NewUserDto;
use crate::domain::users::entities::new_user::NewUser;
use crate::domain::users::repository::{MockUserRepository, UserRepositoryImpl};
use crate::domain::users::service::{MockUserService, UserService, UserServiceImpl};

#[tracing::instrument(name = "Adding a new user", skip(new_user_dto, pool))]
pub async fn save_new_user(
    State(pool): State<PgPool>,
    Json(new_user_dto): Json<NewUserDto>,
) -> StatusCode {
    let new_user: NewUser = match new_user_dto.try_into() {
        Ok(user) => user,
        Err(_) => return StatusCode::BAD_REQUEST,
    };
    //_save_user(pool, new_user).await
    let service = _get_user_service(pool).await;
    match service.save_user(&new_user).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[cfg(not(test))]
async fn _get_user_service(pool: PgPool) -> UserServiceImpl<UserRepositoryImpl> {
    UserServiceImpl::new(UserRepositoryImpl::new(pool))
}

#[cfg(test)]
async fn _get_user_service(pool: PgPool) -> MockUserService {
    MockUserService::new()
}

#[cfg(test)]
mod tests {
    use axum::extract::State;
    use axum::Json;
    use mockall::predicate;
    use sqlx::postgres::PgPoolOptions;

    use crate::configuration::get_configuration;
    use crate::domain::users::dtos::new_user_dto::NewUserDto;
    use crate::domain::users::entities::new_user::NewUser;
    use crate::domain::users::repository::MockUserRepository;
    use crate::domain::users::route::save_new_user;
    use crate::domain::users::service::{MockUserService, UserService};

    #[tokio::test]
    async fn wrong_body_expects_bad_request() {
        let body = r#"
            {
                "name": "Thiago",
                "email": "test@thiago.com",
                "password": "1234512345",
                "roles": ["BASIC", "ADMIN"]
            }
        "#;

        let new_user = NewUser {
            name: "thiago".to_string(),
            email: "".to_string(),
            password: "".to_string(),
            roles: vec![],
        };


        let new_user_dto: NewUserDto = serde_json::from_str(body).unwrap();
        let pg_pool = PgPoolOptions::default().connect_lazy_with(get_configuration().unwrap().database.without_db());
        let mut service_mock = MockUserService::new();
         //   .returning(|_| Ok(()));
        //MockUserService::new().expect_save_user().times(1);
       // MockUserService::expect_save_user(&mut Default::default()).times(1);
        let status_code = save_new_user(State(pg_pool), Json(new_user_dto)).await;
        service_mock.expect_save_user().times(1);

        assert_eq!(status_code, 400);
    }
}
