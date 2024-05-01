use actix_web::{
    web::{get, ServiceConfig},
    HttpResponse,
};
use serde_json::json;

use crate::utils::{
    env::{set_env_vars, ConfVars},
    fs_utils::read_hbs_template,
};
use handlebars::{Handlebars, RenderError};
use log::{debug, error};
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

    let cf: ConfVars = set_env_vars();

    let template_contents = match read_hbs_template(&template_path) {
        Ok(contents) => contents,
        Err(e) => {
            error!(
                "Failed to render contents for edit help:: {}",
                e.to_string()
            );
            HelpDataError::new(e.to_string()).error
        }
    };

    let data = json!({"h": help_data, "conf": cf});

    let yay_help = handlebars.render_template(&template_contents, &data)?;
    Ok(yay_help)
}

async fn help_enterprise() -> Result<String, RenderError> {
    let handlebars = Handlebars::new();

    let template_path = "enterprise_help_html";

    let help_data = HelpData {
        help: "".to_string(),
    };

    let cf: ConfVars = set_env_vars();

    let template_contents = match read_hbs_template(&template_path) {
        Ok(contents) => contents,
        Err(e) => {
            error!(
                "Failed to render contents for edit help_enterprise:: {}",
                e.to_string()
            );
            HelpDataError::new(e.to_string()).error
        }
    };

    let data = json!({"h": help_data, "conf": cf});

    let yay_help = handlebars.render_template(&template_contents, &data)?;
    Ok(yay_help)
}

async fn enterprise_avance_panel() -> Result<String, RenderError> {
    let handlebars = Handlebars::new();

    let template_path = "enterprise_help_avance_html";

    let help_data = HelpData {
        help: "".to_string(),
    };

    let template_contents = match read_hbs_template(&template_path) {
        Ok(contents) => contents,
        Err(e) => {
            error!(
                "Failed to render contents for edit help_enterprise:: {}",
                e.to_string()
            );
            HelpDataError::new(e.to_string()).error
        }
    };

    let cf: ConfVars = set_env_vars();
    let data = json!({"h": help_data, "conf": cf});
    let yay_help = handlebars.render_template(&template_contents, &data)?;
    Ok(yay_help)
}

async fn enterprise_services_panel() -> Result<String, RenderError> {
    let handlebars = Handlebars::new();

    let template_path = "enterprise_help_services_html";
    debug!("template path: {}", template_path);
    let help_data = HelpData {
        help: "".to_string(),
    };

    let template_contents = match read_hbs_template(&template_path) {
        Ok(contents) => contents,
        Err(e) => {
            error!(
                "Failed to render contents for edit help_enterprise:: {}",
                e.to_string()
            );
            HelpDataError::new(e.to_string()).error
        }
    };

    let cf: ConfVars = set_env_vars();
    let data = json!({"h": help_data, "conf": cf});

    let yay_help = handlebars.render_template(&template_contents, &data)?;
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
          .content_type("text/html")
          .body(
            format!("<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load Help: {}</span>",
            e.to_string())
          )
      }
    })
  );

    cfg.route(
    "/help_enterprise",
    get().to(|| async move {
      let yay_help_template = help_enterprise().await;

      match yay_help_template {
        Ok(yht) => HttpResponse::Ok()
          .content_type("text/html")
          .body(yht),
        Err(e) => HttpResponse::Ok()
          .content_type("text/html")
          .body(
            format!("<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load Help: {}</span>",
            e.to_string())
          )
      }
    })
  );

    cfg.route(
    "/help_enterprise_avance",
    get().to(|| async move {
      let yay_help_template = enterprise_avance_panel().await;

      match yay_help_template {
        Ok(yht) => HttpResponse::Ok()
          .content_type("text/html")
          .body(yht),
        Err(e) => HttpResponse::Ok()
          .content_type("text/html")
          .body(
            format!("<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load Help: {}</span>",
            e.to_string())
          )
      }
    })
  );

    cfg.route(
    "/help_enterprise_servicios",
    get().to(|| async move {
      let yay_help_template = enterprise_services_panel().await;

      match yay_help_template {
        Ok(yht) => HttpResponse::Ok()
          .content_type("text/html")
          .body(yht),
        Err(e) => HttpResponse::Ok()
          .content_type("text/html")
          .body(
            format!("<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load Help: {}</span>",
            e.to_string())
          )
      }
    })
  );
}
