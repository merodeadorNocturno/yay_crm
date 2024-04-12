use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
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
    pub password: Option<String>,
    pub date_created: Option<DateTime<Local>>,
    pub date_modified: Option<DateTime<Local>>,
    pub notes: Option<String>,
    pub role_string: Option<String>,
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
    pub notes: Option<String>,
    pub password: Option<String>,
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
            notes: user.notes.clone(),
            password: user.password.clone(),
            role_string: Some(user.role.to_string()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Roles {
    ADMIN,
    EDITOR,
}

impl fmt::Display for Roles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Roles::EDITOR => write!(f, "EDITOR"),
            Roles::ADMIN => write!(f, "ADMIN"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserHandlebarsError {
    pub error: String,
}

impl UserHandlebarsError {
    pub fn new(error: String) -> UserHandlebarsError {
        UserHandlebarsError { error }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserPartials {
    pub date_created: Option<DateTime<Local>>,
    pub password: String,
}
