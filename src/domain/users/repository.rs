use actix_web::web::Data;
use sqlx::PgPool;

use crate::domain::users::entities::new_user::NewUser;


pub trait UserRepository {
    fn new(pool: Data<PgPool>) -> Self;
    async fn insert_user(&self, new_user: &NewUser) -> Result<(), sqlx::Error>;
}

pub struct UserRepositoryImpl {
    pool: Data<PgPool>
}

impl UserRepository for UserRepositoryImpl {
    fn new(pool: Data<PgPool>) -> Self {
        UserRepositoryImpl {
            pool
        }
    }

    #[tracing::instrument(name = "Saving new user details in the database", skip(new_user, self))]
    async fn insert_user(&self, new_user: &NewUser) -> Result<(), sqlx::Error> {
        sqlx::query!(
        r#"
        INSERT INTO users (name, email, password, roles)
              VALUES ($1, $2, $3, $4)
        "#,
        new_user.name,
        new_user.email,
        new_user.password,
        &new_user.roles
    )
            .execute(self.pool.get_ref())
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query: {:?}", e);
                e
            })?;
        Ok(())
    }
}