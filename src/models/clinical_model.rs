use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::sales_model::{SalesFunnel, ServicesOffered};

#[derive(Debug, Deserialize, Serialize)]
pub struct ClinicalUuid {
    pub uuid: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Clinical {
    pub uuid: Option<String>,
    #[validate(length(min = 2, message = "Name does not match valid length"))]
    pub name: String,
    #[validate(length(min = 2, message = "Lastname does not match valid length"))]
    pub last_name: String,
    pub is_company: bool,
    #[validate(length(min = 2, message = "Clinic/Hospital name does not match valid length"))]
    pub clinic_name: Option<String>,
    pub clinic_web: Option<String>,
    #[validate(email)]
    pub clinic_email: Option<String>,
    pub specialty: String,
    #[validate(email)]
    pub email: Option<String>,
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

impl Clinical {
    pub fn new(c_uuid: String, clinical: Clinical) -> Clinical {
        let uuid = Some(c_uuid);
        Clinical { uuid, ..clinical }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClinicalHandlebarsError {
    pub error: String,
}

impl ClinicalHandlebarsError {
    pub fn new(error: String) -> ClinicalHandlebarsError {
        ClinicalHandlebarsError { error }
    }
}
