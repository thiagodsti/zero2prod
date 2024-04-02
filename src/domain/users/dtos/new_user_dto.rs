use serde::{Deserialize, Serialize};

use crate::domain::users::entities::new_user::{NewUser, NewUserBuilder};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUserDto {
    pub name: String,
    pub email: String,
    pub password: String,
    pub roles: Vec<String>,
}

impl TryFrom<NewUserDto> for NewUser {
    type Error = String;
    fn try_from(value: NewUserDto) -> Result<Self, Self::Error> {
        match NewUserBuilder::default()
            .name(value.name)
            .password(value.password)
            .email(value.email)
            .roles(value.roles)
            .build()
        {
            Ok(new_user) => Ok(new_user),
            Err(error) => Err(error.to_string()),
        }
    }
}
