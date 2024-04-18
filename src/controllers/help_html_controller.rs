use actix_web::{
    web::{get, ServiceConfig},
    HttpResponse,
};

use crate::utils::fs_utils::read_hbs_template;
use handlebars::{Handlebars, RenderError};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct HelpData {
    help: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct HelpDataError {
    pub error: String,
}

impl HelpDataError {
    pub fn new(error: String) -> HelpDataError {
        HelpDataError { error }
    }
}

async fn help_html() -> Result<String, RenderError> {
    let handlebars = Handlebars::new();

    let template_path = "help_html";

    let help_data = HelpData {
        help: "".to_string(),
    };

    let template_contents = match read_hbs_template(&template_path) {
        Ok(contents) => contents,
        Err(e) => {
            error!(
                "Failed to render contents for edit enterprise:: {}",
                e.to_string()
            );
            HelpDataError::new(e.to_string()).error
        }
    };

    let yay_help = handlebars.render_template(&template_contents, &help_data)?;
    Ok(yay_help)
}

pub fn help_html_controllers(cfg: &mut ServiceConfig) {
    cfg.route(
    "/help",
    get().to(|| async move {
      let yay_help_template = help_html().await;

      match yay_help_template {
        Ok(yht) => HttpResponse::Ok()
          .content_type("text/html")
        .append_header(("HX-Trigger", "help_table"))
          .body(yht),
        Err(e) => HttpResponse::Ok()
          .content_type("/text/html")
          .body(
            format!("<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load Help: {}</span>",
            e.to_string())
          )
      }
    })
  );
}
