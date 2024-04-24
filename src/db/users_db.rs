use actix_web::web::Data;
use async_trait::async_trait;
use chrono::Local;
use lazy_static::lazy_static;
use log::error;
use surrealdb::{opt::PatchOp, Error};

use crate::db::config::Database;
// use crate::error::user_error::UserError;
// use crate::models::sales_model::IsDeleted;
use crate::constants::connection::set_environment_variable;
use crate::models::users_model::User;
use crate::utils::crud::*;

lazy_static! {
    static ref USERS_TABLE: String = {
        let value = set_environment_variable("USERS_TABLE", "users");
        value.leak().to_string()
    };
}

#[async_trait]
pub trait UsersDB {
    async fn find_all(db: &Data<Database>) -> Option<Vec<User>>;
    async fn find_one(db: &Data<Database>, uuid: String) -> Option<User>;
    async fn add_one(db: &Data<Database>, new_user: User) -> Option<User>;
    async fn update_one(db: &Data<Database>, user: User) -> Option<User>;
    async fn delete_one(db: &Data<Database>, uuid: String) -> Option<User>;
    async fn find_all_non_deleted(db: &Data<Database>) -> Option<Vec<User>>;
    async fn find_all_deleted(db: &Data<Database>) -> Option<Vec<User>>;
}

#[async_trait]
impl UsersDB for Database {
    async fn find_all(db: &Data<Database>) -> Option<Vec<User>> {
        util_find_all(db, &USERS_TABLE).await
    }

    async fn find_one(db: &Data<Database>, uuid: String) -> Option<User> {
        util_find_one(db, uuid, &USERS_TABLE).await
    }

    async fn add_one(db: &Data<Database>, new_user: User) -> Option<User> {
        let uuid = new_user.uuid.clone();

        util_add_one(db, new_user, uuid, "users").await
    }

    async fn update_one(db: &Data<Database>, user: User) -> Option<User> {
        let user_id = user.uuid.clone();
        util_update_one(db, user, user_id, &USERS_TABLE).await
    }

    async fn delete_one(db: &Data<Database>, uuid: String) -> Option<User> {
        let users_table = format!("{}", USERS_TABLE.clone());
        let user: Result<Option<User>, Error> = db
            .client
            .update((users_table, uuid))
            .patch(PatchOp::replace("/deleted", true))
            .patch(PatchOp::replace("/date_modified", Local::now()))
            .await;

        match user {
            Ok(d_user) => match d_user {
                Some(deleted_user) => Some(deleted_user),
                None => None,
            },
            Err(e) => {
                error!("Failed to update user {}", e);
                None
            }
        }
    }

    async fn find_all_non_deleted(db: &Data<Database>) -> Option<Vec<User>> {
        util_find_all_deleted(&db, &USERS_TABLE).await
    }

    async fn find_all_deleted(db: &Data<Database>) -> Option<Vec<User>> {
        let users_table = format!("{}", USERS_TABLE.clone());
        let surreal_query = format!("SELECT * FROM {} WHERE deleted = true", &users_table);

        let users = db.client.query(surreal_query).await;

        match users {
            Ok(mut response) => match response.take(0) {
                Ok(deleted_users) => Some(deleted_users),
                Err(e) => {
                    error!("Failed to retrieve active users {}", e);
                    None
                }
            },
            Err(e) => {
                error!("Failed to retrieve active users {}", e);
                None
            }
        }
    }
}
