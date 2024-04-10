use actix_web::{
    get, patch, post,
    web::{Data, Json, Path, ServiceConfig},
};

use chrono::Local;
use log::error;
use validator::Validate;

use crate::{
    db::{config::Database, enterprise_db::EnterpriseDB},
    error::enterprise_error::EnterpriseError,
    models::enterprise_model::{Enterprise, EnterpriseUuid},
    utils::general_utils::get_uuid,
};

#[get("/enterprise")]
async fn find_all(db: Data<Database>) -> Result<Json<Vec<Enterprise>>, EnterpriseError> {
    let enterprise = Database::find_all(&db).await;

    match enterprise {
        Some(found_enterprise) => Ok(Json(found_enterprise)),
        None => {
            error!("Unable to find any enterprise data");
            Err(EnterpriseError::NoEnterprisesFound)
        }
    }
}

#[get("/enterprise/{uuid}")]
async fn find_one(
    db: Data<Database>,
    uuid: Path<EnterpriseUuid>,
) -> Result<Json<Enterprise>, EnterpriseError> {
    let enterprise_uuid = uuid.into_inner().uuid;
    let enterprise_result = Database::find_one(&db, enterprise_uuid.clone()).await;

    match enterprise_result {
        Some(result) => Ok(Json(result)),
        None => {
            error!("No enterprise found for UUID:: {:?}", &enterprise_uuid);
            Err(EnterpriseError::NoEnterprisesFound)
        }
    }
}

#[post("/enterprise")]
async fn create(
    db: Data<Database>,
    body: Json<Enterprise>,
) -> Result<Json<Enterprise>, EnterpriseError> {
    let is_valid = body.validate();
    let new_enterprise = body.into_inner();

    match is_valid {
        Ok(_) => {
            let new_uuid = get_uuid();
            let my_enterprise =
                Database::add_one(&db, Enterprise::new(String::from(new_uuid), new_enterprise))
                    .await;

            match my_enterprise {
                Some(enterprise_result) => Ok(Json(enterprise_result)),
                None => {
                    error!("Error [POST] /enterprise");
                    Err(EnterpriseError::EnterpriseCreationFailure)
                }
            }
        }
        Err(e) => {
            error!("Error enterprise.create {:?}", e);
            Err(EnterpriseError::EnterpriseCreationFailure)
        }
    }
}

#[patch("/enterprise")]
async fn update_one(
    db: Data<Database>,
    body: Json<Enterprise>,
) -> Result<Json<Enterprise>, EnterpriseError> {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let date_modified = Local::now();
            let my_enterprise = Enterprise {
                uuid: body.uuid.clone(),
                name: body.name.clone(),
                last_name: body.last_name.clone(),
                is_company: body.is_company.clone(),
                line_of_business: body.line_of_business.clone(),
                email: body.email.clone(),
                deleted: body.deleted.clone(),
                fb: body.fb.clone(),
                instagram: body.instagram.clone(),
                linked_in: body.linked_in.clone(),
                tik_tok: body.tik_tok.clone(),
                twitter: body.twitter.clone(),
                first_contact_date: body.first_contact_date.clone(),
                sales_funnel: body.sales_funnel.clone(),
                notes: body.notes.clone(),
                services_offered: body.services_offered.clone(),
                resolution: body.resolution.clone(),
                date_created: body.date_created.clone(),
                date_modified: Some(date_modified),
                created_by: body.created_by.clone(),
                modified_by: body.modified_by.clone(),
            };

            let updated_enterprise = Database::update_one(&db, my_enterprise).await;

            match updated_enterprise {
                Some(enterprise) => Ok(Json(enterprise)),
                None => {
                    error!("Error in enterprise.update_one");
                    Err(EnterpriseError::NoEnterprisesFound)
                }
            }
        }
        Err(e) => {
            error!("Error in enterprise.update_one: {:?}", e);
            Err(EnterpriseError::NoEnterprisesFound)
        }
    }
}

pub fn enterprise_api_controllers(cfg: &mut ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find_one);
    cfg.service(create);
    cfg.service(update_one);
}
