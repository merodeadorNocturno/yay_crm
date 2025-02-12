use actix_cors::Cors;
use actix_web::{middleware, web::Data, App, HttpServer};
use log::{info, warn};

#[macro_use]
extern crate handlebars;

mod constants;
mod controllers;
mod db;
mod error;
mod models;
mod reporting;
mod utils;

use crate::db::config::Database;
use crate::{
    controllers::{
        clinics_api_controller::clinical_api_controllers,
        clinics_html_controller::clinical_html_controllers,
        enterprise_api_controller::enterprise_api_controllers,
        enterprise_html_controller::enterprise_html_controllers,
        help_html_controller::help_html_controllers, school_api_controller::school_api_controllers,
        school_html_controller::school_html_controller,
        users_api_controller::users_api_controllers, users_html_controller::user_html_controllers,
    },
    reporting::telemetry::{get_subscriber, init_subscriber},
    utils::env::{get_cwd, set_env_vars, ConfVars},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let mut builder = Builder::new();
    let subscriber = get_subscriber("yay".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // builder
    //     .filter(None, get_log_level())
    //     .write_style(WriteStyle::Always)
    //     .init();

    match get_cwd() {
        Ok(_) => info!("Successfully retrieved current directory"),
        Err(err) => warn!("Error getting current directory: {}", err),
    }

    let ConfVars {
        server_address,
        server_port,
        ..
    } = set_env_vars();
    let server_address_conf = format!("{server_address}:{server_port}");

    let my_db = Database::init().await.expect("CANT_CONNECT_TO_DB");
    let db_data = Data::new(my_db);

    info!("Welcome to Yay_CRM");

    HttpServer::new(move || {
        let cors = Cors::permissive().max_age(3600);

        App::new()
            .wrap(middleware::NormalizePath::trim())
            .wrap(cors)
            .app_data(db_data.clone())
            .configure(clinical_api_controllers)
            .configure(users_api_controllers)
            .configure(enterprise_api_controllers)
            .configure(user_html_controllers)
            .configure(enterprise_html_controllers)
            .configure(help_html_controllers)
            .configure(clinical_html_controllers)
            .configure(school_api_controllers)
            .configure(school_html_controller)
    })
    .bind(server_address_conf)
    .expect("FAILED TO BIND TO PORT")
    .run()
    .await
}
