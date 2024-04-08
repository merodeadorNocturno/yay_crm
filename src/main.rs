use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use env_logger::{Builder, WriteStyle};
use log::{info, warn, LevelFilter};

mod constants;
mod controllers;
mod db;
mod error;
mod models;
mod utils;

use crate::constants::connection::set_environment_variable;
use crate::db::config::Database;
use crate::{
    controllers::{
        clinical_api_controller::clinical_api_controllers,
        users_api_controller::users_api_controllers,
    },
    utils::env::get_cwd,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = Builder::new();

    builder
        .filter(None, LevelFilter::Debug)
        .write_style(WriteStyle::Always)
        .init();

    match get_cwd() {
        Ok(_) => info!("Successfully retrieved current directory"),
        Err(err) => warn!("Error getting current directory: {}", err),
    }

    let server_address: String = set_environment_variable("SERVER_ADDRESS", "0.0.0.0:8080");

    let my_db = Database::init().await.expect("CANT_CONNECT_TO_DB");
    let db_data = Data::new(my_db);

    info!("Welcome to Yay_CRM");

    HttpServer::new(move || {
        let cors = Cors::permissive().max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(db_data.clone())
            .configure(clinical_api_controllers)
            .configure(users_api_controllers)
    })
    .bind(server_address)
    .expect("FAILED TO BIND TO PORT")
    .run()
    .await
}
