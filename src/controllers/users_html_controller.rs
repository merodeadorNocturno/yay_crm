use actix_web::{
    web::{post, Data, Path, ServiceConfig},
    HttpRequest, HttpResponse,
};
use chrono::Local;
use handlebars::{Handlebars, RenderError};
use log::{error, info};
use serde_json::json;

use crate::{
    db::{config::Database, users_db::UsersDB},
    models::users_model::*,
    utils::{
        env::{set_env_vars, ConfVars},
        fs_utils::read_hbs_template,
        general_utils::{create_role_tags_for_users, get_roles_tag},
    },
};

handlebars_helper!(str_equal: |s1: String, s2: String| s1 == s2);

async fn user_edit(hbs_path: Path<String>, db: Data<Database>) -> Result<String, RenderError> {
    let uuid = hbs_path.into_inner();
    let my_error = format!("Unable to find uuid {}", &uuid).to_string();
    info!("Edit user screen for uuuid:: {}", &uuid);

    let mut template_path = "user_edit";
    println!("{:?}", &template_path);

    let mut handlebars = Handlebars::new();
    handlebars.register_helper("str_equal", Box::new(str_equal));

    let user_from_db: Result<User, UserHandlebarsError> = match Database::find_one(&db, uuid).await
    {
        Some(mut user) => {
            template_path = "user_edit";
            user.role_string = Some(user.role.to_string());
            Ok(user)
        }
        None => {
            error!("Not user found in db");
            template_path = "user_edit";
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
            let user_role_tags = create_role_tags_for_users(user.role.clone());
            let cf: ConfVars = set_env_vars();

            let data = json!({"conf": cf, "u": user, "roles": user_role_tags});
            let render_good = handlebars.render_template(&template_contents, &data)?;
            Ok(render_good)
        }
        Err(e) => {
            let render_error = handlebars.render_template(&template_contents, &e)?;
            Ok(render_error)
        }
    }
}

async fn new_user() -> Result<String, RenderError> {
    let handlebars = Handlebars::new();
    let template_path = "new_user";

    let template_contents = match read_hbs_template(&template_path) {
        Ok(contents) => contents,
        Err(e) => {
            error!("Couldn't render file for new user:: {}", e.to_string(),);
            UserHandlebarsError::new(e.to_string()).error
        }
    };

    let role_tags = get_roles_tag();
    let hb_render = handlebars.render_template(&template_contents, &json!({"roles": role_tags}))?;

    Ok(hb_render)
}

async fn users_table(db: Data<Database>) -> Result<String, RenderError> {
    let template_path = "user_table";
    let handlebars = Handlebars::new();
    let users_from_db = Database::find_all_non_deleted(&db).await;

    let template_contents = match read_hbs_template(&template_path) {
        Ok(contents) => contents,
        Err(e) => {
            error!(
                "<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load user: {}</span>",
                e.to_string()
            );
            UserHandlebarsError::new(e.to_string()).error
        }
    };

    match users_from_db {
        Some(users) => {
            let cf: ConfVars = set_env_vars();

            let data = json!({"conf": cf, "users": users});
            let render_good = handlebars.render_template(&template_contents, &data)?;

            Ok(render_good)
        }
        None => {
            let render_error =
                handlebars.render_template(&template_contents, &"Couldn't get users")?;
            Ok(render_error)
        }
    }
}

async fn users_delete_modal(
    hbs_path: Path<String>,
    db: Data<Database>,
) -> Result<String, RenderError> {
    let uuid = hbs_path.into_inner();
    let handlebars = Handlebars::new();
    let template_path = "user_delete_modal";
    let date_now = Local::now();
    let my_error = format!("Unable to find uuid {}", &uuid).to_string();

    let user_from_db: Result<User, UserHandlebarsError> =
        match Database::find_one(&db, uuid.clone()).await {
            Some(mut user) => {
                user.role_string = Some(user.role.to_string());
                Ok(user)
            }
            None => {
                error!("Not user found in db");
                Err(UserHandlebarsError::new(my_error))
            }
        };

    let template_contents = match read_hbs_template(&template_path) {
        Ok(contents) => contents,
        Err(e) => {
            error!("Couldn't render file for new user:: {}", e.to_string(),);
            UserHandlebarsError::new(e.to_string()).error
        }
    };

    match user_from_db {
        Ok(user) => {
            let render_good = handlebars
                .render_template(&template_contents, &json!({"u": user, "date": date_now}))?;
            Ok(render_good)
        }
        Err(e) => {
            let render_error =
                handlebars.render_template(&template_contents, &json!({"error": &e}))?;
            Ok(render_error)
        }
    }
}

pub fn user_html_controllers(cfg: &mut ServiceConfig) {
    cfg.route(
        "/user_htmx/{uuid}",
        post().to(
            |_req: HttpRequest, hbs_path, db: Data<Database>| async move {
                let user_editor = user_edit(hbs_path, db).await;
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
          let my_users_table = users_table(db).await;
          match my_users_table {
            Ok(ut) => HttpResponse::Ok()
              .content_type("text/html")
              .append_header(("HX-Trigger", "user_table"))
              .body(ut),
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

    cfg.route(
      "/new_user",
      post().to(
        || async move {
          let new_user_form = new_user().await;
          match new_user_form {
            Ok(uf) => HttpResponse::Ok()
              .content_type("text/html")
              .append_header(("HX-Trigger", "user_table"))
              .body(uf),
            Err(e) => HttpResponse::Ok()
              .content_type("text/html")
            .append_header(("HX-Trigger", "user-error"))
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

    cfg.route(
        "/users/delete/{uuid}",
        post().to(|hbs_path, db: Data<Database>| async move {
            let users_delete = users_delete_modal(hbs_path, db).await;
            match users_delete {
              Ok(ud) => HttpResponse::Ok().content_type("text/html").body(ud),
              Err(e) => HttpResponse::Ok().content_type("text/html").body(format!("<span class=\"icon is-small is-left\"><i class=\"fas fa-ban\"></i>Failed to load user: {}</span>",
              e.to_string())),
            }
        }),
    );
}
