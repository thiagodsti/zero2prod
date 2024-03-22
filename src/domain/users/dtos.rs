use serde::Deserialize;

use crate::domain::users::entities::new_user::NewUser;

#[derive(Debug, Deserialize)]
pub struct NewUserDto {
    name: String,
    email: String,
    password: String,
    roles: Vec<String>,
}

impl TryFrom<NewUserDto> for NewUser {
    type Error = String;
    fn try_from(value: NewUserDto) -> Result<Self, Self::Error> {
        Ok(NewUser {
            name: value.name,
            password: value.password,
            email: value.email,
            roles: value.roles,
        })
    }
}
