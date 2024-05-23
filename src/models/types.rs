use serde::{Deserialize, Serialize};

use crate::constants::connection::set_environment_variable;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_host")]
    pub host: String,
    pub port: u16,
    pub client_origin_url: String,
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: set_environment_variable("HOST", &default_host()),
            port: match set_environment_variable("PORT", "6060").parse::<u16>() {
                Ok(parsed) => parsed,
                Err(_) => 6060,
            },
            client_origin_url: set_environment_variable("HOST", "http://localhost:4040"),
        }
    }
}

#[derive(Serialize)]
pub struct ErrorMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    pub message: String,
}

#[derive(Serialize)]
pub struct Metadata {
    pub api: String,
    pub branch: String,
}

#[derive(Serialize)]
pub struct Message {
    pub metadata: Metadata,
    pub text: String,
}
