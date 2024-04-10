use chrono::prelude::*;
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
    #[validate(length(min = 2, message = "Last Name does not match valid length"))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    pub role: Roles,
    pub deleted: bool,
    pub date_created: Option<DateTime<Local>>,
    pub date_modified: Option<DateTime<Local>>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserFromJson {
    #[validate(length(min = 2, message = "Name does not match valid length"))]
    pub name: String,
    #[validate(length(min = 2, message = "Last Name does not match valid length"))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    pub role: Roles,
}

impl User {
    pub fn new(uuid: String, user: UserFromJson) -> User {
        let date_created = Local::now();
        User {
            uuid,
            deleted: false,
            date_created: Some(date_created),
            date_modified: Some(date_created),
            name: user.name.clone(),
            last_name: user.last_name.clone(),
            email: user.email.clone(),
            role: user.role.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Roles {
    ADMIN,
    EDITOR,
}
