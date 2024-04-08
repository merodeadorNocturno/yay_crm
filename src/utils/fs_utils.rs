use log::{error, info};
use std::{fs, io::Error};

use crate::constants::connection::set_environment_variable;

pub fn read_hbs_template(file_name: &str) -> Result<String, Error> {
    info!("Reading from file {}", &file_name);
    let template_pat = set_environment_variable("TEMPLATE_PAT", "./src/static/");
    let full_path = format!("{}{}.hbs", template_pat, &file_name);

    let file_contents = fs::read_to_string(&full_path);

    match file_contents {
        Ok(contents) => Ok(contents),
        Err(e) => {
            error!("Error reading template:: {}", e.to_string());
            Err(Error::from(e))
        }
    }
}
