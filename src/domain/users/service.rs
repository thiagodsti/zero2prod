use mockall::automock;
use sqlx::Error;

use crate::domain::users::entities::new_user::NewUser;
use crate::domain::users::repository::UserRepository;

#[automock]
pub trait UserService {
    async fn save_user(&self, new_user: &NewUser) -> Result<(), Error>;
}

#[derive(Clone)]

pub struct UserServiceImpl<T: UserRepository> {
    repository: T,
}

impl<T> UserServiceImpl<T>
where
    T: UserRepository + 'static,
{
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

impl<T> UserService for UserServiceImpl<T>
where
    T: UserRepository + 'static,
{
    #[tracing::instrument(name = "Adding a new user", skip(new_user, self))]
    async fn save_user(&self, new_user: &NewUser) -> Result<(), Error> {
        self.repository.insert_user(new_user).await
    }
}

#[cfg(test)]
mod tests {
    use claim::{assert_err, assert_ok};
    use sqlx::Error::RowNotFound;

    use crate::domain::users::repository::MockUserRepository;

    use super::*;

    #[tokio::test]
    async fn save_successfully() {
        let mut repository_mock = MockUserRepository::new();
        repository_mock
            .expect_insert_user()
            .times(1)
            .returning(|_| Ok(()));
        let service = UserServiceImpl::new(repository_mock);
        assert_ok!(
            service
                .save_user(&NewUser {
                    name: "thiago".to_string(),
                    email: "".to_string(),
                    password: "".to_string(),
                    roles: vec![],
                })
                .await
        );
    }

    #[tokio::test]
    async fn save_failed() {
        let mut repository_mock = MockUserRepository::new();
        repository_mock
            .expect_insert_user()
            .times(1)
            .returning(|_| Err(RowNotFound));
        let service = UserServiceImpl::new(repository_mock);
        assert_err!(
            service
                .save_user(&NewUser {
                    name: "thiago".to_string(),
                    email: "".to_string(),
                    password: "".to_string(),
                    roles: vec![],
                })
                .await
        );
    }
}
