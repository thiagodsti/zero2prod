use crate::domain::users::repository::UserRepositoryImpl;
use crate::domain::users::route::save_new_user;
use crate::domain::users::service::{UserServiceImpl};
use axum::routing::{get, post};
use axum::Router;
use sqlx::PgPool;

use crate::routes::{health_check};

#[derive(Clone)]
pub struct AppState {
}

pub fn app(pg_pool: PgPool) -> Router {
    let user_service = UserServiceImpl::new(UserRepositoryImpl::new(pg_pool.clone()));
    let state = AppState { };
    Router::new()
        .route("/health_check", get(|| health_check()))
        .route(
            "/users",
            post(move |json| save_new_user(json, user_service.clone())),
        )
        .with_state(state)
}
