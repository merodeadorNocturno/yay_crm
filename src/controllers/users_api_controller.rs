use actix_web::{
    delete, get,
    http::StatusCode,
    patch, post,
    web::{Data, Json, Path, ServiceConfig},
    HttpResponse,
};
use chrono::Local;
use log::{error, info};
use validator::Validate;

use crate::{
    db::{config::Database, users_db::UsersDB},
    error::user_error::UserError,
    models::users_model::{User, UserFromJson, UserUuid},
    utils::{
        general_utils::{get_uuid, shuffle_id},
        message_utils::get_validation_errors,
        pwd::pwd_hasher,
    },
};

#[get("/users")]
async fn find_all(db: Data<Database>) -> Result<HttpResponse, UserError> {
    let user = Database::find_all_non_deleted(&db).await;

    match user {
        Some(found_users) => Ok(HttpResponse::Ok().status(StatusCode::OK).json(found_users)),
        None => {
            error!("Didn't find any User data");
            Ok(HttpResponse::InternalServerError().json(UserUuid {
                uuid: format!("{}", UserError::NoUsersFound),
            }))
        }
    }
}

#[get("/users/deleted")]
async fn find_all_deleted(db: Data<Database>) -> Result<HttpResponse, UserError> {
    let user = Database::find_all_deleted(&db).await;

    match user {
        Some(found_users) => Ok(HttpResponse::Ok().status(StatusCode::OK).json(found_users)),
        None => {
            error!("Didn't find any User data");
            Ok(HttpResponse::NotFound().json(UserUuid {
                uuid: format!("{}", UserError::NoUsersFound),
            }))
        }
    }
}

#[get("/users/{uuid}")]
async fn find_one(db: Data<Database>, user_id: Path<UserUuid>) -> Result<HttpResponse, UserError> {
    let user_uuid = user_id.into_inner().uuid;
    let user_result = Database::find_one(&db, user_uuid.clone()).await;

    match user_result {
        Some(result) => Ok(HttpResponse::Ok().status(StatusCode::OK).json(result)),
        None => {
            error!("No users found for UUID:: {:?}", &user_uuid);
            Ok(HttpResponse::NotFound().json(UserUuid {
                uuid: format!("{}", UserError::NoUsersFound),
            }))
        }
    }
}

#[post("/users")]
async fn create(db: Data<Database>, user: Json<UserFromJson>) -> Result<HttpResponse, UserError> {
    let is_valid = user.validate();
    let new_user = user.into_inner();

    match is_valid {
        Ok(_) => {
            let new_uuid = get_uuid();
            let mut hashed_passwd = "".to_string();
            info!("{}", &hashed_passwd);

            match new_user.password.clone() {
                Some(u_pwd) => match pwd_hasher(u_pwd) {
                    Ok(h_pwd) => {
                        hashed_passwd = h_pwd.clone();
                    }
                    Err(_) => {
                        hashed_passwd = format!("default_passwd_for_user{}", &new_uuid);
                    }
                },
                None => {
                    hashed_passwd = format!("default_passwd_for_user{}", &new_uuid);
                }
            };

            let user_from_json = UserFromJson {
                name: new_user.name.clone(),
                last_name: new_user.last_name.clone(),
                email: new_user.email.clone(),
                role: new_user.role.clone(),
                password: Some(hashed_passwd),
                notes: match new_user.notes {
                    Some(notes) => Some(notes),
                    None => None,
                },
            };

            let my_user =
                Database::add_one(&db, User::new(String::from(new_uuid), user_from_json)).await;

            match my_user {
                Some(user_result) => Ok(HttpResponse::Ok()
                    .insert_header(("HX-Trigger", "user_reload_page"))
                    .status(StatusCode::CREATED)
                    .json(UserUuid {
                        uuid: shuffle_id(user_result.uuid),
                    })),
                None => {
                    error!("Error [POST] /users");
                    Ok(HttpResponse::InternalServerError()
                        .insert_header((
                            "HX-Trigger",
                            format!("{{ \"page_error\": \"Internal server error\" }}"),
                        ))
                        .json(UserUuid {
                            uuid: format!("{}", UserError::UserCreationFailure),
                        }))
                }
            }
        }
        Err(e) => {
            error!("Error users.create {:?}", e);
            let key_errors_vec: Vec<String> = get_validation_errors(&e);

            Ok(HttpResponse::InternalServerError()
                .insert_header((
                    "HX-Trigger",
                    format!("{{ \"page_error\": {:?} }}", key_errors_vec),
                ))
                .json(UserUuid {
                    uuid: format!("{}", UserError::UserCreationFailure),
                }))
        }
    }
}

#[patch("/users")]
async fn update_one(db: Data<Database>, user: Json<User>) -> Result<HttpResponse, UserError> {
    let is_valid = user.validate();

    match is_valid {
        Ok(_) => {
            let uuid_in_db = user.uuid.clone();
            let stored_user = Database::find_one(&db, uuid_in_db).await;
            let date_modified = Local::now();

            let mut mutable_pwd = "".to_string();
            info!("{}", &mutable_pwd);

            match user.password.clone() {
                Some(new_pwd) => match pwd_hasher(new_pwd) {
                    Ok(my_pwd) => {
                        mutable_pwd = my_pwd.clone();
                    }
                    Err(_) => {
                        mutable_pwd = format!("default_passwd_for_user{}", &user.uuid);
                    }
                },
                None => match &stored_user {
                    Some(su) => match &su.password {
                        Some(new_pwd) => {
                            mutable_pwd = new_pwd.to_string();
                        }
                        None => {
                            mutable_pwd = format!("default_passwd_for_user{}", &user.uuid);
                        }
                    },
                    None => {
                        mutable_pwd = format!("default_passwd_for_user{}", &user.uuid);
                    }
                },
            };

            let date_created = match stored_user {
                Some(this_user) => this_user.date_created,
                None => {
                    error!("No date found for UUID:: {:?}", &user.uuid);
                    user.date_created.clone()
                }
            };

            let my_user = User {
                uuid: user.uuid.clone(),
                name: user.name.clone(),
                last_name: user.last_name.clone(),
                email: user.email.clone(),
                role: user.role.clone(),
                deleted: user.deleted.clone(),
                date_created,
                date_modified: Some(date_modified),
                password: Some(mutable_pwd),
                notes: match &user.notes {
                    Some(notes) => Some(String::from(notes)),
                    None => None,
                },
                role_string: Some(user.role.to_string()),
            };

            let updated_user = Database::update_one(&db, my_user).await;

            match updated_user {
                Some(user_result) => Ok(HttpResponse::Ok()
                    .insert_header(("HX-Trigger", "user_reload_page"))
                    .status(StatusCode::OK)
                    .json(UserUuid {
                        uuid: shuffle_id(user_result.uuid),
                    })),
                None => {
                    error!("Error in users.update_one");
                    Ok(HttpResponse::InternalServerError()
                        .insert_header((
                            "HX-Trigger",
                            format!(
                                "{{ \"page_error\": {:?} }}",
                                "Couldn't find school".to_string()
                            ),
                        ))
                        .json(UserUuid {
                            uuid: format!("{}", UserError::NoUsersFound),
                        }))
                }
            }
        }
        Err(e) => {
            error!("Error in users.update_one {:?}", e);
            let key_errors_vec: Vec<String> = get_validation_errors(&e);

            Ok(HttpResponse::InternalServerError()
                .insert_header((
                    "HX-Trigger",
                    format!("{{ \"page_error\": {:?} }}", key_errors_vec),
                ))
                .json(UserUuid {
                    uuid: format!("{}", UserError::UserCreationFailure),
                }))
        }
    }
}

#[delete("/users/{uuid}")]
async fn delete_user(
    db: Data<Database>,
    user_uuid: Path<UserUuid>,
) -> Result<HttpResponse, UserError> {
    let uuid = user_uuid.into_inner().uuid;
    let user_from_db = Database::delete_one(&db, uuid.clone()).await;

    match user_from_db {
        Some(mut user) => {
            user.deleted = true;
            match Database::update_one(&db, user).await {
                Some(deleted_user) => Ok(HttpResponse::Ok()
                    .insert_header(("HX-Trigger", "user_reload_page"))
                    .status(StatusCode::OK)
                    .json(deleted_user)),
                None => {
                    error!("Unable to update user:: {:?}", &uuid);

                    Ok(HttpResponse::InternalServerError()
                        .insert_header((
                            "HX-Trigger",
                            format!(
                                "{{ \"page_error\": {:?} }}",
                                "Couldn't find school".to_string()
                            ),
                        ))
                        .json(UserUuid {
                            uuid: format!("{}", UserError::NoUsersFound),
                        }))
                }
            }
        }
        None => {
            error!("No user found for id:: {:?}", &uuid);

            Ok(HttpResponse::NotFound()
                .insert_header((
                    "HX-Trigger",
                    format!(
                        "{{ \"page_error\": {:?} }}",
                        "Couldn't find school".to_string()
                    ),
                ))
                .json(UserUuid {
                    uuid: format!("{}", UserError::NoUsersFound),
                }))
        }
    }
}

pub fn users_api_controllers(cfg: &mut ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find_all_deleted);
    cfg.service(find_one);
    cfg.service(update_one);
    cfg.service(create);
    cfg.service(delete_user);
}
