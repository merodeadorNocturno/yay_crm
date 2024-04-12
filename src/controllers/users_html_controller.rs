use actix_web::{
    web::{post, Data, Path, ServiceConfig},
    HttpRequest, HttpResponse,
};
use handlebars::{Handlebars, RenderError};
use log::{debug, error, info};
use serde_json::json;

use crate::{
    db::{config::Database, users_db::UsersDB},
    models::users_model::*,
    utils::fs_utils::read_hbs_template,
};

handlebars_helper!(str_equal: |s1: String, s2: String| s1 == s2);

async fn edit_user(hbs_path: Path<String>, db: Data<Database>) -> Result<String, RenderError> {
    let uuid = hbs_path.into_inner();
    debug!(">>>>>>>>>>>>>>>>>> {:?}", &uuid);
    let my_error = format!("Unable to find uuid {}", &uuid).to_string();
    info!("Edit user screen for uuuid:: {}", &uuid);

    let mut template_path = "edit_user";
    debug!("my template path {}", &template_path);
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("str_equal", Box::new(str_equal));

    let user_from_db: Result<User, UserHandlebarsError> = match Database::find_one(&db, uuid).await
    {
        Some(mut user) => {
            template_path = "edit_user";
            user.role_string = Some(user.role.to_string());
            Ok(user)
        }
        None => {
            error!("Not user found in db");
            template_path = "edit_user";
            Err(UserHandlebarsError::new(my_error))
        }
    };

    let template_contents = match read_hbs_template(&template_path) {
        Ok(contents) => contents,
        Err(e) => {
            error!(
                "Failed to render contents for edit user:: {}",
                e.to_string()
            );
            UserHandlebarsError::new(e.to_string()).error
        }
    };

    match user_from_db {
        Ok(user) => {
            let render_good = handlebars.render_template(&template_contents, &user)?;
            Ok(render_good)
        }
        Err(e) => {
            debug!("This is an error in render_container: {:?}", e);
            let render_error = handlebars.render_template(&template_contents, &e)?;
            Ok(render_error)
        }
    }
}

async fn users_table(db: Data<Database>) -> Result<String, RenderError> {
    let template_path = "user_table";
    let handlebars = Handlebars::new();
    let users_from_db = Database::find_all(&db).await;

    let template_contents = match read_hbs_template(&template_path) {
        Ok(contents) => contents,
        Err(e) => {
            error!(
                "Failed to render contents for users table, {:?}",
                e.to_string()
            );
            UserHandlebarsError::new(e.to_string()).error
        }
    };

    match users_from_db {
        Some(users) => {
            let render_good =
                handlebars.render_template(&template_contents, &json!({ "users": users }))?;
            Ok(render_good)
        }
        None => {
            debug!("This is an error in render_container:");
            let render_error =
                handlebars.render_template(&template_contents, &"Couldn't get users")?;
            Ok(render_error)
        }
    }
}

pub fn user_html_controllers(cfg: &mut ServiceConfig) {
    cfg.route(
        "/user_htmx/{uuid}",
        post().to(
            |_req: HttpRequest, hbs_path, db: Data<Database>| async move {
                let user_editor = edit_user(hbs_path, db).await;
                match user_editor {
                    Ok(ue) => HttpResponse::Ok().content_type("text/html").body(ue),
                    Err(e) => HttpResponse::Ok()
                        .content_type("text/html")
                        .body(
                          format!(
                            "<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load user: {}</span>",
                            e.to_string()
                          )
                        ),
                }
            },
        ),
    );

    cfg.route(
      "/user_htmx",
      post().to(
        |db: Data<Database>| async move {
          let users_table = users_table(db).await;
          match users_table {
            Ok(ut) => HttpResponse::Ok().content_type("text/html").body(ut),
            Err(e) => HttpResponse::Ok()
              .content_type("text/html")
              .body(
                format!(
                  "<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load users: {}</span>",
                  e.to_string()
                )
              )
          }
        }
      ),
    );
}
