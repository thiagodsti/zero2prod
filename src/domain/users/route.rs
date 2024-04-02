use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

use crate::domain::users::dtos::new_user_dto::NewUserDto;
use crate::domain::users::entities::new_user::NewUser;
use crate::domain::users::service::UserService;
use crate::startup::AppState;

#[tracing::instrument(name = "Adding a new user", skip(new_user_dto, _state, user_service))]
pub async fn save_new_user(
    State(_state): State<AppState>,
    Json(new_user_dto): Json<NewUserDto>,
    user_service: impl UserService,
) -> StatusCode {
    let new_user: NewUser = match new_user_dto.try_into() {
        Ok(user) => user,
        Err(_) => return StatusCode::BAD_REQUEST,
    };
    match user_service.save_user(&new_user).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[cfg(test)]
mod tests {
    use axum::extract::State;
    use axum::Json;
    use sqlx::postgres::PgPoolOptions;
    use sqlx::Error;

    use crate::configuration::get_configuration;
    use crate::domain::users::dtos::new_user_dto::NewUserDto;
    use crate::domain::users::route::save_new_user;
    use crate::domain::users::service::MockUserService;
    use crate::startup::AppState;

    #[tokio::test]
    async fn wrong_body_expects_bad_request() {
        let body = r#"
            {
                "name": "Thiago",
                "email": "",
                "password": "",
                "roles": ["BASIC", "ADMIN"]
            }
        "#;

        let new_user_dto: NewUserDto = serde_json::from_str(body).unwrap();
        let pg_pool = PgPoolOptions::default()
            .connect_lazy_with(get_configuration().unwrap().database.without_db());
        let mut service_mock = MockUserService::new();

        service_mock.expect_save_user().never();
        let status_code = save_new_user(
            State(AppState { pool: pg_pool }),
            Json(new_user_dto),
            service_mock,
        )
        .await;
        assert_eq!(status_code, 400);
    }

    #[tokio::test]
    async fn bad_return_from_database_expects_internal_server_error() {
        let body = r#"
            {
                "name": "Thiago",
                "email": "thiago@test.com",
                "password": "1234512345",
                "roles": ["BASIC", "ADMIN"]
            }
        "#;

        let new_user_dto: NewUserDto = serde_json::from_str(body).unwrap();
        let pg_pool = PgPoolOptions::default()
            .connect_lazy_with(get_configuration().unwrap().database.without_db());
        let mut service_mock = MockUserService::new();

        service_mock
            .expect_save_user()
            .returning(|_| Err(Error::PoolClosed))
            .times(1);
        let status_code = save_new_user(
            State(AppState { pool: pg_pool }),
            Json(new_user_dto),
            service_mock,
        )
        .await;
        assert_eq!(status_code, 500);
    }
}
