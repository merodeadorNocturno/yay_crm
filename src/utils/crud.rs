use actix_web::web::Data;
use log::error;

use serde::{de::DeserializeOwned, Serialize};
use surrealdb::{sql, Error};

use crate::db::config::Database;

// use super::general_utils::create_select_query;

pub async fn util_find_all<T: DeserializeOwned>(
    db: &Data<Database>,
    table_name: &str,
) -> Option<Vec<T>> {
    let result = db.client.select(table_name).await;

    match result {
        Ok(all_users) => Some(all_users),
        Err(e) => {
            error!("Error {}.find_all:: {:?}", &table_name, e);
            None
        }
    }
}

pub async fn util_find_one<T: DeserializeOwned>(
    db: &Data<Database>,
    uuid: String,
    table_name: &str,
) -> Option<T> {
    let t_by_uuid: Result<Option<T>, Error> = db.client.select((table_name, uuid)).await;

    match t_by_uuid {
        Ok(uuid_t) => uuid_t,
        Err(e) => {
            error!("Error {}.find_one:: {:?}", &table_name, e);
            None
        }
    }
}

pub async fn util_add_one<T: DeserializeOwned + Serialize>(
    db: &Data<Database>,
    t: T,
    uuid: String,
    table_name: &str,
) -> Option<T> {
    let created_t = db.client.create((table_name, uuid)).content(t).await;

    match created_t {
        Ok(t_record) => t_record,
        Err(e) => {
            error!("Error {}.add_one:: {:?}", table_name, e);
            None
        }
    }
}

pub async fn util_update_one<T: DeserializeOwned + Serialize>(
    db: &Data<Database>,
    t: T,
    uuid: String,
    table_name: &str,
) -> Option<T> {
    let t_id = uuid.clone();
    let t_to_update: Result<Option<T>, Error> = db.client.select((table_name, &t_id)).await;

    match t_to_update {
        Ok(found_t) => match found_t {
            Some(_t) => {
                let updated_t: Result<Option<T>, Error> =
                    db.client.update((table_name, &t_id)).merge(t).await;

                match updated_t {
                    Ok(updated_t_values) => updated_t_values,
                    Err(e) => {
                        error!("Error {}.find_one:: {:?}", table_name, e);
                        None
                    }
                }
            }
            None => None,
        },
        Err(e) => {
            error!("Error {}: {:?}", table_name, e);
            None
        }
    }
}

// pub async fn util_query_table<T: DeserializeOwned + Serialize>(
//     db: &Database,
//     table_name: &str,
//     search_by: &str,
//     where_item_equals: &str,
// ) -> Option<T> {
//     let mut res = db.client.query("SELECT * FROM users").await?;
//     let query_result: Option<T> = res.take(0)?;
//     query_result
// }
