use mockall::automock;
use sqlx::PgPool;

use crate::domain::users::entities::new_user::NewUser;

#[automock]
pub trait UserRepository {
    async fn insert_user(&self, new_user: &NewUser) -> Result<(), sqlx::Error>;
}

#[derive(Clone)]
pub struct UserRepositoryImpl {
    pool: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserRepository for UserRepositoryImpl {
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
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use sqlx::{query};
    use crate::domain::users::entities::new_user::NewUserBuilder;
    use crate::domain::users::repository::{UserRepository, UserRepositoryImpl};
    use crate::utils::db_test_connection::db_config;

    #[tokio::test]
    async fn save_successfully() {
        let pg_pool = db_config::configure_database().await;
        let user_repository = UserRepositoryImpl::new(pg_pool.clone());

        let new_user = NewUserBuilder::default()
            .name(String::from("Thiago"))
            .email(String::from("thiago@test.com"))
            .password(String::from("1234512345"))
            .roles(vec!(String::from("BASIC"),String::from("ADMIN")))
            .build()
            .unwrap();
        user_repository.insert_user(&new_user).await.unwrap();

        let saved = query!("SELECT email, name FROM users",)
        .fetch_one(&pg_pool)
        .await
        .expect("Failed to fetch saved users.");
        assert_eq!(saved.email, "thiago@test.com");
        assert_eq!(saved.name, "Thiago");
    }


}
