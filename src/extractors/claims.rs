use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError, ResponseError},
    http::{StatusCode, Uri},
    Error, FromRequest, HttpResponse,
};
use actix_web_httpauth::{
    extractors::bearer::BearerAuth, headers::www_authenticate::bearer::Bearer,
};
use awc::{
    error::{JsonPayloadError, SendRequestError},
    Client,
};
use derive_more::Display;
use jsonwebtoken::{
    decode, decode_header,
    jwk::{AlgorithmParameters, JwkSet},
    Algorithm, DecodingKey, Validation,
};
use serde::Deserialize;
use std::{collections::HashSet, future::Future, pin::Pin};
use thiserror::Error;

use crate::constants::connection::set_environment_variable;
use crate::models::types::ErrorMessage;

#[derive(Debug, Clone, Deserialize)]
pub struct Auth0Config {
    audience: String,
    domain: String,
}

impl Default for Auth0Config {
    fn default() -> Self {
        Auth0Config {
            audience: set_environment_variable("AUTH0_AUDIENCE", "https://crm.yayleads.mx"),
            domain: set_environment_variable("AUTH0_DOMAIN", "dev-zv75zriia3jcgnej.us.auth0.com"),
        }
    }
}

#[derive(Error, Debug, Display)]
enum ClientError {
    #[display(fmt = "authentication")]
    Authentication(actix_web_httpauth::extractors::AuthenticationError<Bearer>),
    #[display(fmt = "decode")]
    Decode(jsonwebtoken::errors::Error),
    #[display(fmt = "not_found")]
    NotFound(String),
    #[display(fmt = "unsupported_algorithm")]
    UnsupportedAlgorithm(AlgorithmParameters),
    #[display(fmt = "Failed to fetch JWKS")]
    JWKSError(SendRequestError),
    #[display(fmt = "Invalid JWKS format")]
    InvalidJWKSError(JsonPayloadError),
}

impl ResponseError for ClientError {
    fn error_response(&self) -> HttpResponse<awc::body::BoxBody> {
        match self {
            Self::Authentication(_) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: None,
                error_description: None,
                message: "Requires auth".to_string(),
            }),
            Self::Decode(_) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: Some("invalid_token".to_string()),
                error_description: Some(
                    "Authorization header value must follow this format: Bearer acces-token"
                        .to_string(),
                ),
                message: "Bad credentials".to_string(),
            }),
            Self::NotFound(msg) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: Some("invalid_token".to_string()),
                error_description: Some(msg.to_string()),
                message: "Bad credentials".to_string(),
            }),
            Self::UnsupportedAlgorithm(algo) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: Some("invalid_token".to_string()),
                error_description: Some(format!(
                    "Unsupported encryption algorithm expected RSA, got {:?}",
                    algo
                )),
                message: "Bad credentials".to_string(),
            }),
            Self::JWKSError(_) => {
                ErrorInternalServerError("Internal Server Error").error_response()
            }
            Self::InvalidJWKSError(_) => ErrorBadRequest("Invalid JWKS data").error_response(),
        }
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }
}

#[derive(Debug, Deserialize)]
pub struct Claims {
    _permissions: Option<HashSet<String>>,
}

impl FromRequest for Claims {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let config = req.app_data::<Auth0Config>().unwrap().clone();
        let extractor = BearerAuth::extract(req);

        Box::pin(async move {
            let credentials = extractor.await.map_err(ClientError::Authentication)?;
            let token = credentials.token();
            let header = decode_header(token).map_err(ClientError::Decode)?;
            let kid = header.kid.ok_or_else(|| {
                ClientError::NotFound("kid not found in token header".to_string())
            })?;
            let domain = config.domain.as_str();
            let jwks: JwkSet = Client::new()
                .get(
                    Uri::builder()
                        .scheme("https")
                        .authority(domain)
                        .path_and_query("/.well-known/jwks.json")
                        .build()
                        .unwrap(),
                )
                .send()
                .await
                .map_err(ClientError::JWKSError)?
                .json()
                .await
                .map_err(ClientError::InvalidJWKSError)?;

            let jwk = jwks
                .find(&kid)
                .ok_or_else(|| ClientError::NotFound("No JWK found for kid".to_string()))?;

            match jwk.clone().algorithm {
                AlgorithmParameters::RSA(ref rsa) => {
                    let mut validation = Validation::new(Algorithm::RS256);
                    validation.set_audience(&[config.audience]);
                    validation.set_issuer(&[Uri::builder()
                        .scheme("https")
                        .authority(domain)
                        .path_and_query("/")
                        .build()
                        .unwrap()]);
                    let key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e)
                        .map_err(ClientError::Decode)?;
                    let new_token =
                        decode::<Claims>(token, &key, &validation).map_err(ClientError::Decode)?;

                    Ok(new_token.claims)
                }
                algorithm => Err(ClientError::UnsupportedAlgorithm(algorithm).into()),
            }
        }) // Box::pin
    }
}
