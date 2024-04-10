use actix_web::web::Data;
use async_trait::async_trait;

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
}
