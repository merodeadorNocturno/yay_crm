use actix_web::{
    get, patch, post,
    web::{Data, Json, Path, ServiceConfig},
};

use log::error;
use validator::Validate;

use crate::{
    db::{config::Database, users_db::UsersDB},
    error::user_error::UserError,
};

use crate::{
    models::users_model::{User, UserUuid},
    utils::general_utils::get_uuid,
};

#[get("/users")]
async fn find_all(db: Data<Database>) -> Result<Json<Vec<User>>, UserError> {
    let user = Database::find_all(&db).await;

    match user {
        Some(found_users) => Ok(Json(found_users)),
        None => {
            error!("Didn't find any User data");
            Err(UserError::NoUsersFound)
        }
    }
}

#[get("/users/{uuid}")]
async fn find_one(db: Data<Database>, user_id: Path<UserUuid>) -> Result<Json<User>, UserError> {
    let user_uuid = user_id.into_inner().uuid;
    let user_result = Database::find_one(&db, user_uuid.clone()).await;

    match user_result {
        Some(result) => Ok(Json(result)),
        None => {
            error!("No users found for UUID:: {:?}", &user_uuid);
            Err(UserError::NoUsersFound)
        }
    }
}

#[post("/users")]
async fn create(db: Data<Database>, user: Json<User>) -> Result<Json<User>, UserError> {
    let is_valid = user.validate();
    let new_user = user.into_inner();

    match is_valid {
        Ok(_) => {
            let new_uuid = get_uuid();
            let my_user = Database::add_one(&db, User::new(String::from(new_uuid), new_user)).await;

            match my_user {
                Some(user_result) => Ok(Json(user_result)),
                None => {
                    error!("Error [POST] /users");
                    Err(UserError::UserCreationFailure)
                }
            }
        }
        Err(e) => {
            error!("Error users.create {:?}", e);
            Err(UserError::UserCreationFailure)
        }
    }
}

#[patch("/users")]
async fn update_one(db: Data<Database>, user: Json<User>) -> Result<Json<User>, UserError> {
    let is_valid = user.validate();

    match is_valid {
        Ok(_) => {
            let my_user = User {
                uuid: user.uuid.clone(),
                name: user.name.clone(),
                last_name: user.last_name.clone(),
                email: user.email.clone(),
                role: user.role.clone(),
                deleted: user.deleted.clone(),
            };

            let updated_user = Database::update_one(&db, my_user).await;

            match updated_user {
                Some(user_result) => Ok(Json(user_result)),
                None => {
                    error!("Error in users.update_one");
                    Err(UserError::NoUsersFound)
                }
            }
        }
        Err(e) => {
            error!("Error in users.update_one {:?}", e);
            Err(UserError::NoUsersFound)
        }
    }
}

pub fn users_api_controllers(cfg: &mut ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find_one);
    cfg.service(update_one);
    cfg.service(create);
}
