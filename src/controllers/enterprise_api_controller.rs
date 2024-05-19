use actix_web::{
    delete, get,
    http::StatusCode,
    patch, post,
    web::{Data, Json, Path, ServiceConfig},
    HttpResponse,
};
use chrono::Local;
use log::error;
use validator::Validate;

use crate::{
    db::{config::Database, enterprise_db::EnterpriseDB},
    error::enterprise_error::EnterpriseError,
    models::enterprise_model::{Enterprise, EnterpriseUuid},
    utils::general_utils::{get_uuid, shuffle_id},
};

#[get("/enterprises")]
async fn find_all(db: Data<Database>) -> Result<HttpResponse, EnterpriseError> {
    let enterprise = Database::find_all_active(&db).await;

    match enterprise {
        Some(found_enterprise) => Ok(HttpResponse::Ok()
            .status(StatusCode::OK)
            .json(found_enterprise)),
        None => {
            error!("Unable to find any enterprise data");
            Ok(HttpResponse::NotFound().json(EnterpriseUuid {
                uuid: format!("{}", EnterpriseError::NoEnterprisesFound),
            }))
        }
    }
}

#[get("/enterprises/{uuid}")]
async fn find_one(
    db: Data<Database>,
    uuid: Path<EnterpriseUuid>,
) -> Result<HttpResponse, EnterpriseError> {
    let enterprise_uuid = uuid.into_inner().uuid;
    let enterprise_result = Database::find_one(&db, enterprise_uuid.clone()).await;

    match enterprise_result {
        Some(result) => Ok(HttpResponse::Ok().status(StatusCode::OK).json(result)),
        None => {
            error!("No enterprise found for UUID:: {:?}", &enterprise_uuid);
            Ok(HttpResponse::NotFound().json(EnterpriseUuid {
                uuid: format!("{}", EnterpriseError::NoEnterprisesFound),
            }))
        }
    }
}

#[post("/enterprises")]
async fn create(
    db: Data<Database>,
    body: Json<Enterprise>,
) -> Result<HttpResponse, EnterpriseError> {
    let is_valid = body.validate();
    let date_created = Local::now();
    let mut new_enterprise = body.into_inner();
    new_enterprise.date_created = Some(date_created.clone());
    new_enterprise.date_modified = Some(date_created.clone());
    new_enterprise.first_contact_date = Some(date_created.clone());

    match is_valid {
        Ok(_) => {
            let new_uuid = get_uuid();
            let my_enterprise =
                Database::add_one(&db, Enterprise::new(String::from(new_uuid), new_enterprise))
                    .await;

            match my_enterprise {
                Some(enterprise_result) => Ok(HttpResponse::Ok()
                    .insert_header(("HX-Trigger", "enterprise_create"))
                    .status(StatusCode::OK)
                    .json(EnterpriseUuid {
                        uuid: match enterprise_result.uuid {
                            Some(this_uuid) => this_uuid,
                            None => "".to_string(),
                        },
                    })),
                None => {
                    error!("Error [POST] /enterprise");
                    Ok(HttpResponse::InternalServerError().json(EnterpriseUuid {
                        uuid: "Error".to_string(),
                    }))
                }
            }
        }
        Err(e) => {
            error!("Error enterprise.create {:?}", e);
            Ok(HttpResponse::InternalServerError().json(EnterpriseUuid {
                uuid: format!("{}", EnterpriseError::EnterpriseCreationFailure),
            }))
        }
    }
}

#[patch("/enterprises")]
async fn update_one(
    db: Data<Database>,
    body: Json<Enterprise>,
) -> Result<HttpResponse, EnterpriseError> {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let uuid_id_db = match body.uuid.clone() {
                Some(c_id) => c_id,
                None => String::from("forbidden"),
            };

            let stored_enterprise = Database::find_one(&db, uuid_id_db).await;
            let date_modified = Local::now();

            let e_cloned = stored_enterprise.clone();

            let date_created = match stored_enterprise {
                Some(enterprise) => enterprise.date_created,
                None => {
                    error!("No date found for UUID:: {:?}", body.uuid.clone());
                    Some(Local::now())
                }
            };

            let my_enterprise = Enterprise {
                uuid: body.uuid.clone(),
                name: body.name.clone(),
                last_name: body.last_name.clone(),
                is_company: body.is_company.clone(),
                company_name: body.company_name.clone(),
                line_of_business: body.line_of_business.clone(),
                phone: match body.phone.clone() {
                    Some(my_phone) => Some(my_phone),
                    None => None,
                },
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
                date_created,
                date_modified: Some(date_modified),
                created_by: match e_cloned {
                    Some(my_enterprise) => my_enterprise.created_by,
                    None => Some(String::from("n/a")),
                },
                modified_by: body.modified_by.clone(),
            };

            let updated_enterprise = Database::update_one(&db, my_enterprise).await;

            match updated_enterprise {
                Some(enterprise) => Ok(HttpResponse::Ok()
                    .insert_header(("HX-Trigger", "enterprise_update"))
                    .status(StatusCode::OK)
                    .json(EnterpriseUuid {
                        uuid: match enterprise.uuid {
                            Some(this_uuid) => shuffle_id(this_uuid),
                            None => "".to_string(),
                        },
                    })),
                None => {
                    error!("Error in enterprise.update_one");
                    Ok(HttpResponse::InternalServerError().json(EnterpriseUuid {
                        uuid: "Error".to_string(),
                    }))
                }
            }
        }
        Err(e) => {
            error!("Error in enterprise.update_one: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(EnterpriseUuid {
                uuid: format!("{}", EnterpriseError::NoEnterprisesFound),
            }))
        }
    }
}

#[get("/enterprises/deleted")]
async fn find_all_deleted(db: Data<Database>) -> Result<HttpResponse, EnterpriseError> {
    let enterprises = Database::find_all_deleted(&db).await;

    match enterprises {
        Some(deleted_enterprises) => Ok(HttpResponse::Ok()
            .insert_header(("HX-Trigger", "enterprise_fad"))
            .status(StatusCode::OK)
            .json(deleted_enterprises)),
        None => {
            error!("Didnt' find any deleted enterprises");
            Ok(HttpResponse::NotFound().json(EnterpriseUuid {
                uuid: format!("{}", EnterpriseError::NoEnterprisesFound),
            }))
        }
    }
}

#[delete("/enterprises/{uuid}")]
async fn delete_one(
    db: Data<Database>,
    uuid: Path<EnterpriseUuid>,
) -> Result<HttpResponse, EnterpriseError> {
    let enterprise_uuid = uuid.into_inner().uuid;
    let enterprise_from_db: Option<Enterprise> =
        Database::delete_one(&db, enterprise_uuid.clone()).await;

    match enterprise_from_db {
        Some(mut enterprise) => {
            enterprise.deleted = true;
            match Database::update_one(&db, enterprise).await {
                Some(_) => Ok(HttpResponse::Ok()
                    .insert_header(("HX-Trigger", "enterprise_delete"))
                    .status(StatusCode::OK)
                    .json(EnterpriseUuid {
                        uuid: enterprise_uuid.to_string(),
                    })),
                None => {
                    error!("Unable to delete enterprise:: {:?}", &enterprise_uuid);
                    Ok(HttpResponse::InternalServerError().json(EnterpriseUuid {
                        uuid: "".to_string(),
                    }))
                }
            }
        }
        None => {
            error!("Unable to update school :: {:?}", &enterprise_uuid);
            Ok(HttpResponse::NotFound().json(EnterpriseUuid {
                uuid: "".to_string(),
            }))
        }
    }
}

pub fn enterprise_api_controllers(cfg: &mut ServiceConfig) {
    cfg.service(create);
    cfg.service(delete_one);
    cfg.service(find_all);
    cfg.service(find_all_deleted);
    cfg.service(find_one);
    cfg.service(update_one);
}
