use actix_web::web::Data;
use async_trait::async_trait;
use log::error;

use surrealdb::Error;

use crate::db::config::Database;
use crate::models::users_model::User;
use crate::utils::crud::*;

const USERS_TABLE: &str = "users";

#[async_trait]
pub trait UsersDB {
    async fn find_all(db: &Data<Database>) -> Option<Vec<User>>;
    async fn find_one(db: &Data<Database>, uuid: String) -> Option<User>;
    async fn add_one(db: &Data<Database>, new_user: User) -> Option<User>;
    async fn update_one(db: &Data<Database>, user: User) -> Option<User>;
}

#[async_trait]
impl UsersDB for Database {
    async fn find_all(db: &Data<Database>) -> Option<Vec<User>> {
        util_find_all(db, USERS_TABLE).await
    }

    async fn find_one(db: &Data<Database>, uuid: String) -> Option<User> {
        util_find_one(db, uuid, USERS_TABLE).await
    }

    async fn add_one(db: &Data<Database>, new_user: User) -> Option<User> {
        let uuid = new_user.uuid.clone();

        util_add_one(db, new_user, uuid, "users").await
    }

    async fn update_one(db: &Data<Database>, user: User) -> Option<User> {
        let user_id = user.uuid.clone();
        util_update_one(db, user, user_id, USERS_TABLE).await
    }
}
