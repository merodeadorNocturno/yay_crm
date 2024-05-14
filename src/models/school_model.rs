use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::sales_model::{SalesFunnel, SchoolLevel, ServicesOffered};

#[derive(Debug, Deserialize, Serialize)]
pub struct SchoolUuid {
    pub uuid: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct School {
    pub uuid: Option<String>,
    #[validate(length(min = 2, message = "Name does not match valid length"))]
    pub name: String,
    #[validate(length(min = 2, message = "Lastname does not match valid length"))]
    pub last_name: String,
    #[validate(length(min = 2, message = "School name does not match valid length"))]
    pub school_name: String,
    pub school_level: Vec<SchoolLevel>,
    #[validate(email)]
    pub email: String,
    pub phone: String,
    pub deleted: bool,
    #[validate(url)]
    pub fb: Option<String>,
    pub instagram: Option<String>,
    pub linked_in: Option<String>,
    pub tik_tok: Option<String>,
    pub twitter: Option<String>,
    pub first_contact_date: Option<DateTime<Local>>,
    pub sales_funnel: SalesFunnel,
    pub notes: String,
    pub services_offered: Vec<ServicesOffered>,
    pub date_created: Option<DateTime<Local>>,
    pub date_modified: Option<DateTime<Local>>,
    pub created_by: Option<String>,
    pub modified_by: Option<String>,
}

impl School {
    pub fn new(s_uuid: String, school: School) -> School {
        let uuid = Some(s_uuid);
        School { uuid, ..school }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SchoolHandlebarsError {
    pub error: String,
}

impl SchoolHandlebarsError {
    pub fn new(error: String) -> SchoolHandlebarsError {
        SchoolHandlebarsError { error }
    }
}
