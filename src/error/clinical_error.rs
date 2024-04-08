use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, HttpServer, ResponseError,
};

use derive_more::Display;

#[derive(Debug, Display)]
pub enum ClinicalError {
    NoClinicalsFound = 0,
    ClinicalCreationFailure = 1,
}

impl ResponseError for ClinicalError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ClinicalError::NoClinicalsFound => StatusCode::NOT_FOUND,
            ClinicalError::ClinicalCreationFailure => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
