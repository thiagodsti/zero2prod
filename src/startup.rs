use crate::domain::users::repository::UserRepositoryImpl;
use crate::domain::users::route::save_new_user;
use crate::domain::users::service::UserServiceImpl;
use axum::routing::{get, post};
use axum::Router;
use sqlx::PgPool;

use crate::routes::{health_check, subscribe};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub fn app(pg_pool: PgPool) -> Router {
    let user_service = UserServiceImpl::new(UserRepositoryImpl::new(pg_pool.clone()));
    let state = AppState { pool: pg_pool };
    Router::new()
        .route("/health_check", get(|| health_check()))
        .route("/subscriptions", post(subscribe))
        .route(
            "/users",
            post(move |state, json| save_new_user(state, json, user_service.clone())),
        )
        .with_state(state)
}
