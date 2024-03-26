use actix_web::web::Data;
use sqlx::{Error, PgPool};

use crate::domain::users::entities::new_user::NewUser;
use crate::domain::users::repository::{UserRepository, UserRepositoryImpl};


pub trait UserService {
    fn new(pool: Data<PgPool>) -> Self;
    async fn save_user(&self, new_user: &NewUser) -> Result<(), Error>;
}

pub struct UserServiceImpl {
    repository: UserRepositoryImpl,
}

impl UserService for UserServiceImpl {
    fn new(pool: Data<PgPool>) -> Self {
        Self {
            repository: UserRepositoryImpl::new(pool),
        }
    }

    #[tracing::instrument(name = "Adding a new user", skip(new_user, self))]
    async fn save_user(&self, new_user: &NewUser) -> Result<(), Error> {
        self.repository.insert_user(new_user).await
    }
}

