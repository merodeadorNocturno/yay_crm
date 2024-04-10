use serde::Deserialize;

use crate::models::users_model::Roles;

#[derive(Deserialize)]
pub struct Projection {
    pub name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub role: Option<Roles>,
    pub deleted: Option<bool>,
}
