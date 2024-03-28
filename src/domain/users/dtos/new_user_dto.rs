use serde::{Deserialize, Serialize};

use crate::domain::users::entities::new_user::NewUser;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUserDto {
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) roles: Vec<String>,
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
