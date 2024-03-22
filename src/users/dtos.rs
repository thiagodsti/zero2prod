use crate::users::entities::new_user::NewUser;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NewUserDto {
    name: String,
    username: String,
    password: String,
    email: String,
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
