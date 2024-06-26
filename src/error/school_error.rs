use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};

use derive_more::Display;

#[derive(Debug, Display)]
pub enum SchoolError {
    NoSchoolsFound = 0,
    SchoolCreationFailure = 1,
}
impl ResponseError for SchoolError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            SchoolError::NoSchoolsFound => StatusCode::NOT_FOUND,
            SchoolError::SchoolCreationFailure => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
