use actix_web::web::Data;
use async_trait::async_trait;
use chrono::Local;
use log::error;
use surrealdb::{opt::PatchOp, Error};

use crate::{
    db::config::Database,
    models::school_model::School,
    utils::{crud::*, general_utils::get_uuid},
};

const SCHOOL_TABLE: &str = "schools";

#[async_trait]
pub trait SchoolDB {
    async fn find_all(db: &Data<Database>) -> Option<Vec<School>>;
    async fn find_one(db: &Data<Database>, uuid: String) -> Option<School>;
    async fn add_one(db: &Data<Database>, new_school: School) -> Option<School>;
    async fn update_one(db: &Data<Database>, old_school: School) -> Option<School>;
    async fn find_all_active(db: &Data<Database>) -> Option<Vec<School>>;
    async fn find_all_deleted(db: &Data<Database>) -> Option<Vec<School>>;
    async fn delete_one(db: &Data<Database>, uuid: String) -> Option<School>;
}

#[async_trait]
impl SchoolDB for Database {
    async fn find_all(db: &Data<Database>) -> Option<Vec<School>> {
        util_find_all(db, SCHOOL_TABLE).await
    }
    async fn find_one(db: &Data<Database>, uuid: String) -> Option<School> {
        util_find_one(db, uuid, SCHOOL_TABLE).await
    }
    async fn add_one(db: &Data<Database>, new_school: School) -> Option<School> {
        let uuid = new_school.uuid.clone();
        let school_id = match uuid {
            Some(this_uuid) => this_uuid,
            None => get_uuid(),
        };
        util_add_one(db, new_school, school_id, SCHOOL_TABLE).await
    }
    async fn update_one(db: &Data<Database>, old_school: School) -> Option<School> {
        let school_uuid = old_school.uuid.clone();
        let school_id = match school_uuid {
            Some(this_uuid) => this_uuid,
            None => get_uuid(),
        };

        util_update_one(db, old_school, school_id, SCHOOL_TABLE).await
    }

    async fn find_all_active(db: &Data<Database>) -> Option<Vec<School>> {
        util_find_all_non_deleted(&db, SCHOOL_TABLE).await
    }

    async fn find_all_deleted(db: &Data<Database>) -> Option<Vec<School>> {
        let query = format!("SELECT * FROM {} WHERE deleted = true", SCHOOL_TABLE);
        let schools = db.client.query(query).await;

        match schools {
            Ok(mut result) => match result.take(0) {
                Ok(deleted_schools) => Some(deleted_schools),
                Err(e) => {
                    error!("Failed to retrieve deleted schools {}", e);
                    None
                }
            },
            Err(e) => {
                error!("Failed to retrieve deleted schools {}", e);
                None
            }
        }
    }

    async fn delete_one(db: &Data<Database>, uuid: String) -> Option<School> {
        let school_exists: Result<Option<School>, Error> =
            db.client.select((SCHOOL_TABLE, uuid.clone())).await;

        match school_exists {
            Ok(_) => {
                let school: Result<Option<School>, Error> = db
                    .client
                    .update((SCHOOL_TABLE, &uuid))
                    .patch(PatchOp::replace("/deleted", true))
                    .patch(PatchOp::replace("/date_modified", Local::now()))
                    .await;

                match school {
                    Ok(deleted_school) => match deleted_school {
                        Some(ds) => Some(ds),
                        None => None,
                    },
                    Err(e) => {
                        error!("Failed to delete school: {}", e);
                        None
                    }
                }
            }
            Err(e) => {
                error!("Failed to delete school: {}", e);
                None
            }
        }
    }
}
