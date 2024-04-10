use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};

use derive_more::Display;

#[derive(Debug, Display)]
pub enum EnterpriseError {
    NoEnterprisesFound = 0,
    EnterpriseCreationFailure = 1,
}

impl ResponseError for EnterpriseError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            EnterpriseError::NoEnterprisesFound => StatusCode::NOT_FOUND,
            EnterpriseError::EnterpriseCreationFailure => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
