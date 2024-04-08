use actix_web::web::Data;
use async_trait::async_trait;

use crate::db::config::Database;
use crate::models::clinical_model::Clinical;
use crate::utils::crud::*;

const CLINICAL_TABLE: &str = "clinical";

#[async_trait]
pub trait ClinicalDB {
    async fn find_all(db: &Data<Database>) -> Option<Vec<Clinical>>;
    async fn find_one(db: &Data<Database>, uuid: String) -> Option<Clinical>;
    async fn add_one(db: &Data<Database>, new_clinical: Clinical) -> Option<Clinical>;
    async fn update_one(db: &Data<Database>, clinical: Clinical) -> Option<Clinical>;
}

#[async_trait]
impl ClinicalDB for Database {
    async fn find_all(db: &Data<Database>) -> Option<Vec<Clinical>> {
        util_find_all(db, "clinical").await
    }

    async fn find_one(db: &Data<Database>, uuid: String) -> Option<Clinical> {
        util_find_one(db, uuid, CLINICAL_TABLE).await
    }

    async fn add_one(db: &Data<Database>, new_clinical: Clinical) -> Option<Clinical> {
        let uuid = new_clinical.uuid.clone();
        util_add_one(db, new_clinical, uuid, CLINICAL_TABLE).await
    }

    async fn update_one(db: &Data<Database>, clinical: Clinical) -> Option<Clinical> {
        let clinical_id = clinical.uuid.clone();

        util_update_one(db, clinical, clinical_id, CLINICAL_TABLE).await
    }
}
