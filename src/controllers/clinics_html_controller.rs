use actix_web::{
    web::{post, Data, Path, ServiceConfig},
    HttpRequest, HttpResponse,
};
use handlebars::{Handlebars, RenderError};
use log::{error, info};
use serde_json::json;

use crate::db::{clinical_db::ClinicalDB, config::Database};
use crate::models::clinical_model::*;
use crate::utils::{
    env::{set_env_vars, ConfVars},
    fs_utils::read_hbs_template,
    general_utils::{create_option_tags_info_for_services_and_funnel, get_options_and_services},
};

handlebars_helper!(str_equal: |s1: String, s2: String| s1 == s2);

async fn clinical_edit(hbs_path: Path<String>, db: Data<Database>) -> Result<String, RenderError> {
    let uuid = hbs_path.into_inner();
    let my_error = format!("Unable to find uuid {}", &uuid).to_string();
    info!("Edit user screen for uuid:: {}", &uuid);

    let mut handlebars = Handlebars::new();
    handlebars.register_helper("str_equal", Box::new(str_equal));

    let mut template_path = "clinical_edit";

    let clinical_from_db: Result<Clinical, ClinicalHandlebarsError> =
        match Database::find_one(&db, uuid).await {
            Some(clinic) => Ok(clinic),
            None => {
                error!("Not clinics found in db");
                template_path = "clinical_edit";
                Err(ClinicalHandlebarsError::new(my_error))
            }
        };

    let template_contents = match read_hbs_template(&template_path) {
        Ok(contents) => contents,
        Err(e) => {
            error!(
                "Failed to render contents for edit enterprise:: {}",
                e.to_string()
            );
            ClinicalHandlebarsError::new(e.to_string()).error
        }
    };

    match clinical_from_db {
        Ok(this_clinic) => {
            let (services_tag, funnel_tag) = create_option_tags_info_for_services_and_funnel(
                this_clinic.services_offered.clone(),
                this_clinic.sales_funnel.clone(),
            );

            let cf: ConfVars = set_env_vars();
            let data = json!({
              "conf": cf,
              "services_tag": services_tag,
              "sales_funnel": funnel_tag,
              "c": this_clinic
            });

            let render_good = handlebars.render_template(&template_contents, &data)?;
            Ok(render_good)
        }
        Err(e) => {
            let render_error = handlebars.render_template(&template_contents, &e)?;
            Ok(render_error)
        }
    }
}

async fn clinical_new() -> Result<String, RenderError> {
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("str_equal", Box::new(str_equal));

    let template_path = "clinical_new";

    let template_contents = match read_hbs_template(&template_path) {
        Ok(contents) => contents,
        Err(e) => {
            error!(
                "Couldn't render file for new enterprise:: {}",
                e.to_string()
            );
            ClinicalHandlebarsError::new(e.to_string()).error
        }
    };

    let (services_tag, funnel_tag) = get_options_and_services();
    let cf: ConfVars = set_env_vars();
    let data = json!({ "conf": cf, "services_tag": services_tag, "sales_funnel": funnel_tag});

    let handlebars_render = handlebars.render_template(&template_contents, &data)?;

    Ok(handlebars_render)
}

async fn clinical_table(db: Data<Database>) -> Result<String, RenderError> {
    let template_path = "clinical_table";
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("str_equal", Box::new(str_equal));

    let clinics_from_db = Database::find_all(&db).await;

    let template_contents = match read_hbs_template(&template_path) {
        Ok(contents) => contents,
        Err(e) => {
            error!(
          "<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load user: {}</span>",
          e.to_string()
        );

            ClinicalHandlebarsError::new(e.to_string()).error
        }
    };

    match clinics_from_db {
        Some(these_clinics) => {
            let cf: ConfVars = set_env_vars();
            let data = json!({ "conf": cf, "clinics": these_clinics });

            let render = handlebars.render_template(&template_contents, &data)?;
            Ok(render)
        }
        None => {
            let render_error =
                handlebars.render_template(&template_contents, &"Couldn't get clinics")?;
            Ok(render_error)
        }
    }
}

pub fn clinical_html_controllers(cfg: &mut ServiceConfig) {
    cfg.route(
      "/clinics/edit/{uuid}",
      post().to(
          |_req: HttpRequest, hbs_path, db: Data<Database>| async move {
              let clinic_editor_screen = clinical_edit(hbs_path, db).await;
              match clinic_editor_screen {
                  Ok(ces) => HttpResponse::Ok().content_type("text/html")
                    .body(ces),
                  Err(e) => HttpResponse::Ok()
                      .content_type("text/html")
                      .append_header(("HX-Trigger", "error_enterprise_table"))
                      .body(
                        format!(
                          "<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load enterprise: {}</span>",
                          e.to_string()
                        )
                      ),
              }
          },
      ),
  );

    cfg.route(
    "/clinics_table",
    post().to(
      |db: Data<Database>| async move {
        let my_enterprise_table = clinical_table(db).await;

        match my_enterprise_table {
          Ok(et) => HttpResponse::Ok()
            .content_type("text/html")
            .append_header(("HX-Trigger", "activate_navbar_element"))
            .body(et),
          Err(e) => HttpResponse::Ok()
            .content_type("text/html")
            .append_header(("HX-Trigger", "error_enterprise_table"))
            .body(
              format!("<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load Enterprise: {}</span>",
              e.to_string())
            )
        }
      }
    ),
  );

    cfg.route(
        "/new_clinic",
        post().to(|| async move {
            let new_clinical_editor = clinical_new().await;

            match new_clinical_editor {
              Ok(new_enterprise) => HttpResponse::Ok()
                .content_type("text/html")
                .body(new_enterprise),
              Err(e) => HttpResponse::Ok()
                .content_type("text/html")
                .append_header(("HX-Trigger", "error_clinical_table"))
                .body(
                  format!("<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load Enterprise: {}</span>",
                  e.to_string())
                )
            }
        }),
    );
}
