use actix_web::web::Data;
use async_trait::async_trait;
use chrono::Local;
use log::error;
use surrealdb::opt::PatchOp;
use surrealdb::Error;

// use surrealdb::opt::Resource;
// use surrealdb::Notification;
// use surrealdb::{engine::remote::ws::Client, method::Stream, Error, Notification};

use crate::db::config::Database;
use crate::models::enterprise_model::Enterprise;
use crate::utils::{crud::*, general_utils::get_uuid};

const ENTERPRISE_TABLE: &str = "enterprise";

#[async_trait]
pub trait EnterpriseDB {
    async fn find_all(db: &Data<Database>) -> Option<Vec<Enterprise>>;
    async fn find_one(db: &Data<Database>, uuid: String) -> Option<Enterprise>;
    async fn add_one(db: &Data<Database>, new_enterprise: Enterprise) -> Option<Enterprise>;
    async fn update_one(db: &Data<Database>, enterprise: Enterprise) -> Option<Enterprise>;
    async fn find_all_active(db: &Data<Database>) -> Option<Vec<Enterprise>>;
    async fn find_all_deleted(db: &Data<Database>) -> Option<Vec<Enterprise>>;
    async fn delete_one(db: &Data<Database>, uuid: String) -> Option<Enterprise>;
}

#[async_trait]
impl EnterpriseDB for Database {
    async fn find_all(db: &Data<Database>) -> Option<Vec<Enterprise>> {
        util_find_all(db, ENTERPRISE_TABLE).await
    }

    async fn find_one(db: &Data<Database>, uuid: String) -> Option<Enterprise> {
        util_find_one(db, uuid, ENTERPRISE_TABLE).await
    }

    async fn add_one(db: &Data<Database>, new_enterprise: Enterprise) -> Option<Enterprise> {
        let uuid = new_enterprise.uuid.clone();
        let my_id = match uuid {
            Some(this_uuid) => this_uuid,
            None => get_uuid(),
        };
        util_add_one(db, new_enterprise, my_id, ENTERPRISE_TABLE).await
    }

    async fn update_one(db: &Data<Database>, enterprise: Enterprise) -> Option<Enterprise> {
        let enterprise_id = enterprise.uuid.clone();
        let my_id = match enterprise_id {
            Some(this_uuid) => this_uuid,
            None => get_uuid(),
        };
        util_update_one(db, enterprise, my_id, ENTERPRISE_TABLE).await
    }

    async fn find_all_active(db: &Data<Database>) -> Option<Vec<Enterprise>> {
        util_find_all_non_deleted(&db, ENTERPRISE_TABLE).await
    }

    async fn find_all_deleted(db: &Data<Database>) -> Option<Vec<Enterprise>> {
        let query = format!("SELECT * FROM {} WHERE deleted = true", ENTERPRISE_TABLE);
        let enterprises = db.client.query(query).await;

        match enterprises {
            Ok(mut result) => match result.take(0) {
                Ok(deleted_enterprises) => Some(deleted_enterprises),
                Err(e) => {
                    error!("Failed to retrieve deleted {}", e);
                    None
                }
            },
            Err(e) => {
                error!("Failed to retrieve deleted schools {}", e);
                None
            }
        }
    }

    async fn delete_one(db: &Data<Database>, uuid: String) -> Option<Enterprise> {
        let enterprise_exists: Result<Option<Enterprise>, Error> =
            db.client.select((ENTERPRISE_TABLE, uuid.clone())).await;

        match enterprise_exists {
            Ok(_) => {
                let my_enterprise: Result<Option<Enterprise>, Error> = db
                    .client
                    .update((ENTERPRISE_TABLE, &uuid))
                    .patch(PatchOp::replace("/deleted", true))
                    .patch(PatchOp::replace("/date_modified", Local::now()))
                    .await;

                match my_enterprise {
                    Ok(deleted_enterprise) => match deleted_enterprise {
                        Some(de) => Some(de),
                        None => None,
                    },
                    Err(e) => {
                        error!("Failed to delete enterprise:: {}", e);
                        None
                    }
                }
            }
            Err(e) => {
                error!("Failed to delete enterprise: {}", e);
                None
            }
        }
    }
}
