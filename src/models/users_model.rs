use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize)]
pub struct UserUuid {
    pub uuid: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct User {
    pub uuid: String,
    #[validate(length(min = 2, message = "Name does not match valid length"))]
    pub name: String,
    #[validate(length(min = 2, message = "Name does not match valid length"))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    pub role: Roles,
    pub deleted: bool,
}

impl User {
    pub fn new(uuid: String, user: User) -> User {
        User { uuid, ..user }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Roles {
    ADMIN,
    EDITOR,
}
