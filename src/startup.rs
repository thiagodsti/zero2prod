use axum::Router;
use axum::routing::{get, post};
use sqlx::{PgPool};
use crate::domain::users::route::save_new_user;

use crate::routes::{health_check, subscribe};


pub fn app(pg_pool: PgPool) -> Router {
    Router::new()
        .route("/health_check", get(|| health_check()))
        .route("/subscriptions", post(subscribe))
        .route("/users", post(save_new_user))
        .with_state(pg_pool)

}
