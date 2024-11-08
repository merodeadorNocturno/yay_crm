use log::{info, LevelFilter};
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
    pub hbs_target_address: String,
    pub hbs_target_port: String,
}

pub fn set_env_vars() -> ConfVars {
    let server_address = set_environment_variable("SERVER_ADDRESS", "0.0.0.0");
    let server_port = set_environment_variable("SERVER_PORT", "8080");
    let server_protocol = set_environment_variable("SERVER_PROTOCOL", "http");
    let hbs_target_address = set_environment_variable("HBS_TARGET_ADDRESS", "0.0.0.0");
    let mut hbs_target_port = set_environment_variable("HBS_TARGET_PORT", "8080");

    if hbs_target_port == "80".to_string() || hbs_target_port != "".to_string() {
        hbs_target_port = "".to_string();
    } else {
        hbs_target_port = format!(":{}", &hbs_target_port);
    }

    ConfVars {
        server_address,
        server_port,
        server_protocol,
        hbs_target_address,
        hbs_target_port,
    }
}

pub fn get_log_level() -> LevelFilter {
    let log_level = set_environment_variable("RUST_LOG", "debug");

    let level_filter = match log_level.as_str() {
        "off" => LevelFilter::Off,
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Debug,
    };

    level_filter
}
