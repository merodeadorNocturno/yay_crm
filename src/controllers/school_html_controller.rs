use actix_web::{
    web::{post, Data, Path, ServiceConfig},
    HttpRequest, HttpResponse,
};
use chrono::Local;
use handlebars::{Handlebars, RenderError};
use log::{debug, error, info};
use serde_json::json;

use crate::{
    db::{config::Database, school_db::SchoolDB},
    models::{
        sales_model::{GeneralTags, SchoolLevel},
        school_model::*,
    },
    utils::{
        env::{set_env_vars, ConfVars},
        fs_utils::read_hbs_template,
        general_utils::{
            create_option_tags_info_for_services_and_funnel, create_school_level_tags,
            get_options_and_services, get_school_level_tags,
        },
    },
};

handlebars_helper!(str_equal: |s1: String, s2: String| s1 == s2);
handlebars_helper!(levels: |v1: Vec<SchoolLevel>| {
  let slt = create_school_level_tags(v1);
  json!(slt)
});

async fn school_edit(hbs_path: Path<String>, db: Data<Database>) -> Result<String, RenderError> {
    let uuid = hbs_path.into_inner();
    let my_error = format!("Unable to find uuid {}", &uuid);
    info!("Edit user screen for uuid:: {}", &uuid);

    let mut handlebars = Handlebars::new();
    handlebars.register_helper("str_equal", Box::new(str_equal));

    let mut template_path = "school_edit";

    let school_from_db: Result<School, SchoolHandlebarsError> =
        match Database::find_one(&db, uuid).await {
            Some(school) => Ok(school),
            None => {
                error!("No schools found in db");
                template_path = "clinical_edit";
                Err(SchoolHandlebarsError::new(my_error))
            }
        };

    let template_contents = match read_hbs_template(&template_path) {
        Ok(contents) => contents,
        Err(e) => {
            error!(
                "Failed to render contents for edit school:: {}",
                e.to_string()
            );
            SchoolHandlebarsError::new(e.to_string()).error
        }
    };

    match school_from_db {
        Ok(this_school) => {
            let (services_tag, funnel_tag) = create_option_tags_info_for_services_and_funnel(
                this_school.services_offered.clone(),
                this_school.sales_funnel.clone(),
            );

            let level_tag = create_school_level_tags(this_school.school_level.clone());

            let cf: ConfVars = set_env_vars();
            let data = json!({
              "conf": cf,
              "services_tag": services_tag,
              "sales_funnel": funnel_tag,
              "school_level_tag": level_tag,
              "s": this_school,
            });

            let render = handlebars.render_template(&template_contents, &data)?;
            Ok(render)
        }
        Err(e) => {
            let render_error = handlebars.render_template(&template_contents, &e)?;
            Ok(render_error)
        }
    }
}

async fn school_table(db: Data<Database>) -> Result<String, RenderError> {
    let template_path = "school_table";
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("str_equal", Box::new(str_equal));
    handlebars.register_helper("levels", Box::new(levels));

    let schools_from_db = Database::find_all_active(&db).await;

    let template_contents = match read_hbs_template(&template_path) {
        Ok(contents) => contents,
        Err(e) => {
            error!(
        "<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load user: {}</span>",
        e.to_string()
      );

            SchoolHandlebarsError::new(e.to_string()).error
        }
    };

    let my_cf: ConfVars = set_env_vars();

    match schools_from_db {
        Some(these_schools) => {
            let mut tag_vectors: Vec<GeneralTags<School>> = Vec::new();

            for school in these_schools {
                let (services_tag, funnel_tag) = create_option_tags_info_for_services_and_funnel(
                    school.services_offered.clone(),
                    school.sales_funnel.clone(),
                );

                tag_vectors.push(GeneralTags::<School> {
                    section: school,
                    funnel_tag,
                    services_tag,
                });
            }

            let data = json!({"conf": my_cf, "schools": tag_vectors});

            let render = handlebars.render_template(&template_contents, &data)?;
            Ok(render)
        }
        None => {
            let data = json!({"conf": my_cf, "error": "Unable to fetch schools"});
            let render_error = handlebars.render_template(&template_contents, &data)?;
            Ok(render_error)
        }
    }
}

async fn school_new() -> Result<String, RenderError> {
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("str_equal", Box::new(str_equal));

    let template_path = "school_new";

    let template_contents = match read_hbs_template(&template_path) {
        Ok(contents) => contents,
        Err(e) => {
            error!(
                "Couldn't render file for new enterprise:: {}",
                e.to_string()
            );
            SchoolHandlebarsError::new(e.to_string()).error
        }
    };

    let (services_tag, funnel_tag) = get_options_and_services();
    let school_level_tags = get_school_level_tags();

    let cf: ConfVars = set_env_vars();

    let data = json!({
      "conf": cf,
      "school": school_level_tags,
      "funnel": funnel_tag,
      "services": services_tag,
      "date": format!("{}", Local::now()),
    });

    let handlebars_render = handlebars.render_template(&template_contents, &data)?;

    Ok(handlebars_render)
}

pub fn school_html_controller(cfg: &mut ServiceConfig) {
    cfg.route(
        "/htmx/schools/edit/{uuid}",
        post().to(
            |_req: HttpRequest, hbs_path, db: Data<Database>| async move {
                let school_editor_screen = school_edit(hbs_path, db).await;

                match school_editor_screen {
                  Ok(ses) => HttpResponse::Ok().content_type("text/html")
                    .body(ses),
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
      "/htmx/schools/table",
      post().to(
        |db: Data<Database>| async move {
          let htmx_school_table = school_table(db).await;

          match htmx_school_table {
            Ok(hst) => HttpResponse::Ok()
              .content_type("text/html")
              .append_header(("HX-Trigger", "activate_navbar_element"))
              .body(hst),
            Err(e) => HttpResponse::Ok()
              .content_type("text/html")
              .append_header(("HX-Trigger", "error_school_table"))
              .body(
                format!("<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load Enterprise: {}</span>",
                e.to_string())
              )
          }
        }
      ),
    );

    cfg.route(
        "/htmx/schools/new",
        post().to(
            || async move {
                let school_editor_screen = school_new().await;

                match school_editor_screen {
                  Ok(ses) => HttpResponse::Ok().content_type("text/html")
                    .body(ses),
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
}
