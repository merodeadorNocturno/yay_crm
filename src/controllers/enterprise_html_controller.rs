use actix_web::{
    web::{post, Data, Path, ServiceConfig},
    HttpRequest, HttpResponse,
};
use handlebars::{Handlebars, RenderError};
use log::{error, info};

use crate::db::{config::Database, enterprise_db::EnterpriseDB};
use crate::models::enterprise_model::*;
use crate::utils::fs_utils::read_hbs_template;

handlebars_helper!(str_equal: |s1: String, s2: String| s1 == s2);

async fn edit_enterprise(
    hbs_path: Path<String>,
    db: Data<Database>,
) -> Result<String, RenderError> {
    let uuid = hbs_path.into_inner();
    let my_error = format!("Unable to find uuid {}", &uuid).to_string();
    info!("Edit user screen for uuuid:: {}", &uuid);

    let mut handlebars = Handlebars::new();
    handlebars.register_helper("str_equal", Box::new(str_equal));

    let mut template_path = "edit_enterprise";

    let enterprise_from_db: Result<Enterprise, EnterpriseHandlebarsError> =
        match Database::find_one(&db, uuid).await {
            Some(enterprise) => Ok(enterprise),
            None => {
                error!("Not enterprise found in db");
                template_path = "edit_user";
                Err(EnterpriseHandlebarsError::new(my_error))
            }
        };

    let template_contents = match read_hbs_template(&template_path) {
        Ok(contents) => contents,
        Err(e) => {
            error!(
                "Failed to render contents for edit enterprise:: {}",
                e.to_string()
            );
            EnterpriseHandlebarsError::new(e.to_string()).error
        }
    };

    match enterprise_from_db {
        Ok(user) => {
            let render_good = handlebars.render_template(&template_contents, &user)?;
            Ok(render_good)
        }
        Err(e) => {
            let render_error = handlebars.render_template(&template_contents, &e)?;
            Ok(render_error)
        }
    }
}

pub fn enterprise_html_controllers(cfg: &mut ServiceConfig) {
    cfg.route(
      "/enterprise_htmx/{uuid}",
      post().to(
          |_req: HttpRequest, hbs_path, db: Data<Database>| async move {
              let user_editor = edit_enterprise(hbs_path, db).await;
              match user_editor {
                  Ok(ue) => HttpResponse::Ok().content_type("text/html").body(ue),
                  Err(e) => HttpResponse::Ok()
                      .content_type("text/html")
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
