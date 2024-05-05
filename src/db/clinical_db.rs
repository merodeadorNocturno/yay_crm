use actix_web::web::Data;
use async_trait::async_trait;
use chrono::Local;
use log::error;
use surrealdb::{opt::PatchOp, Error};

use crate::db::config::Database;
use crate::models::clinical_model::Clinical;
use crate::utils::crud::*;
use crate::utils::general_utils::get_uuid;

const CLINICAL_TABLE: &str = "clinical";

#[async_trait]
pub trait ClinicalDB {
    async fn find_all(db: &Data<Database>) -> Option<Vec<Clinical>>;
    async fn find_one(db: &Data<Database>, uuid: String) -> Option<Clinical>;
    async fn add_one(db: &Data<Database>, new_clinical: Clinical) -> Option<Clinical>;
    async fn update_one(db: &Data<Database>, clinical: Clinical) -> Option<Clinical>;
    async fn find_all_non_deleted(db: &Data<Database>) -> Option<Vec<Clinical>>;
    async fn find_all_deleted(db: &Data<Database>) -> Option<Vec<Clinical>>;
    async fn delete_one(db: &Data<Database>, uuid: String) -> Option<Clinical>;
}

#[async_trait]
impl ClinicalDB for Database {
    async fn find_all(db: &Data<Database>) -> Option<Vec<Clinical>> {
        util_find_all(db, CLINICAL_TABLE).await
    }

    async fn find_one(db: &Data<Database>, uuid: String) -> Option<Clinical> {
        util_find_one(db, uuid, CLINICAL_TABLE).await
    }

    async fn add_one(db: &Data<Database>, new_clinical: Clinical) -> Option<Clinical> {
        let uuid = new_clinical.uuid.clone();
        let my_id = match uuid {
            Some(this_uuid) => this_uuid,
            None => get_uuid(),
        };
        util_add_one(db, new_clinical, my_id, CLINICAL_TABLE).await
    }

    async fn update_one(db: &Data<Database>, clinical: Clinical) -> Option<Clinical> {
        let clinical_id = clinical.uuid.clone();
        let my_id = match clinical_id {
            Some(this_uuid) => this_uuid,
            None => get_uuid(),
        };
        util_update_one(db, clinical, my_id, CLINICAL_TABLE).await
    }

    async fn find_all_non_deleted(db: &Data<Database>) -> Option<Vec<Clinical>> {
        util_find_all_non_deleted(&db, CLINICAL_TABLE).await
    }

    async fn find_all_deleted(db: &Data<Database>) -> Option<Vec<Clinical>> {
        let surreal_query = format!("SELECT * FROM {} WHERE deleted = true", CLINICAL_TABLE);
        let clinics = db.client.query(surreal_query).await;

        match clinics {
            Ok(mut response) => match response.take(0) {
                Ok(deleted_clinics) => Some(deleted_clinics),
                Err(e) => {
                    error!("Failed to retrieve deleted clinics {}", e);
                    None
                }
            },
            Err(e) => {
                error!("Failed to retrieve deleted clinics {}", e);
                None
            }
        }
    }

    async fn delete_one(db: &Data<Database>, uuid: String) -> Option<Clinical> {
        let clinic_exists: Result<Option<Clinical>, Error> =
            db.client.select((CLINICAL_TABLE, uuid.clone())).await;

        if let Ok(Some(_)) = clinic_exists {
            let clinic: Result<Option<Clinical>, Error> = db
                .client
                .update((CLINICAL_TABLE, uuid))
                .patch(PatchOp::replace("/deleted", true))
                .patch(PatchOp::replace("/date_modified", Local::now()))
                .await;

            match clinic {
                Ok(deleted_clinic) => match deleted_clinic {
                    Some(dc) => Some(dc),
                    None => None,
                },
                Err(e) => {
                    error!("Failed to delete clinic:: {}", e);
                    None
                }
            }
        } else {
            None
        }
    }
}
