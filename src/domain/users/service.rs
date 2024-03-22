use actix_web::web;
use sqlx::{Error, PgPool};

use crate::domain::users::entities::new_user::NewUser;
use crate::domain::users::repository::insert_user;

#[tracing::instrument(name = "Adding a new user", skip(new_user, pool))]
pub async fn save_user(new_user: &NewUser, pool: web::Data<PgPool>) -> Result<(), Error> {
    insert_user(&pool, new_user).await
}
