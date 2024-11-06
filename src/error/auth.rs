use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServiceErrorLocal {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "JWKSFetchError")]
    JWKSFetchError,
}

impl ResponseError for ServiceErrorLocal {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            ServiceErrorLocal::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceErrorLocal::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceErrorLocal::JWKSFetchError => {
                HttpResponse::InternalServerError().json("Could not fetch JWKS")
            }
        }
    }
}
