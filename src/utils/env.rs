use log::info;
use serde::{Deserialize, Serialize};
use std::{env, io};

use crate::constants::connection::set_environment_variable;

pub fn get_cwd() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    info!("Current working directory: {}", current_dir.display());

    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfVars {
    pub server_address: String,
    pub server_port: String,
    pub server_protocol: String,
}

pub fn set_env_vars() -> ConfVars {
    ConfVars {
        server_address: set_environment_variable("SERVER_ADDRESS", "0.0.0.0"),
        server_port: set_environment_variable("SERVER_PORT", "8080"),
        server_protocol: set_environment_variable("SERVER_PROTOCOL", "http"),
    }
}
