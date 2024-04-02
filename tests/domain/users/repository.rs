use crate::utils::startup::configure_database;
use sqlx::query;
use zero2prod::domain::users::entities::new_user::NewUserBuilder;
use zero2prod::domain::users::repository::{UserRepository, UserRepositoryImpl};

#[tokio::test]
async fn save_successfully() {
    let pg_pool = configure_database().await;

    let user_repository = UserRepositoryImpl::new(pg_pool.clone());

    let new_user = NewUserBuilder::default()
        .name(String::from("Thiago"))
        .email(String::from("thiago@test.com"))
        .password(String::from("1234512345"))
        .roles(vec![String::from("BASIC"), String::from("ADMIN")])
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
