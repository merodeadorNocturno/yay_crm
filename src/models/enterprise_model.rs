use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::sales_model::{SalesFunnel, ServicesOffered};

#[derive(Debug, Deserialize, Serialize)]
pub struct EnterpriseUuid {
    pub uuid: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Enterprise {
    pub uuid: Option<String>,
    #[validate(length(min = 2, message = "Name does not match valid length"))]
    pub name: String,
    #[validate(length(min = 2, message = "Lastname does not match valid length"))]
    pub last_name: String,
    pub is_company: bool,
    #[validate(length(min = 2, message = "Lastname does not match valid length"))]
    pub company_name: Option<String>,
    pub line_of_business: Option<String>,
    pub phone: Option<String>,
    #[validate(email)]
    pub email: String,
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
    pub resolution: Option<String>,
    pub date_created: Option<DateTime<Local>>,
    pub date_modified: Option<DateTime<Local>>,
    pub created_by: Option<String>,
    pub modified_by: Option<String>,
}

impl Enterprise {
    pub fn new(id: String, enterprise: Enterprise) -> Enterprise {
        let uuid = Some(id);
        Enterprise { uuid, ..enterprise }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnterpriseHandlebarsError {
    pub error: String,
}

impl EnterpriseHandlebarsError {
    pub fn new(error: String) -> EnterpriseHandlebarsError {
        EnterpriseHandlebarsError { error }
    }
}
