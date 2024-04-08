use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::sales_model::{SalesFunnel, ServicesOffered};

#[derive(Debug, Deserialize)]
pub struct ClinicalUuid {
    pub uuid: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct Clinical {
    pub uuid: String,
    #[validate(length(min = 2, message = "Name does not match valid length"))]
    pub name: String,
    #[validate(length(min = 2, message = "Name does not match valid length"))]
    pub last_name: String,
    pub is_company: bool,
    pub specialty: String,
    #[validate(email)]
    pub email: String,
    pub deleted: bool,
    #[validate(url)]
    pub fb: Option<String>,
    pub instagram: Option<String>,
    pub linked_in: Option<String>,
    pub tik_tok: Option<String>,
    pub twitter: Option<String>,
    pub first_contact_date: String,
    pub sales_funnel: SalesFunnel,
    pub notes: String,
    pub services_offered: Vec<ServicesOffered>,
    pub resolution: Option<String>,
}

impl Clinical {
    pub fn new(uuid: String, clinical: Clinical) -> Clinical {
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
