use actix_web::{
    delete, get, patch, post,
    web::{Data, Json, Path, ServiceConfig},
};

use chrono::Local;
use log::error;
use validator::Validate;

use crate::db::{clinical_db::ClinicalDB, config::Database};
use crate::error::clinical_error::ClinicalError;
use crate::{
    models::clinical_model::{Clinical, ClinicalUuid},
    utils::general_utils::get_uuid,
};

#[get("/clinical")]
async fn find_all(db: Data<Database>) -> Result<Json<Vec<Clinical>>, ClinicalError> {
    let clinical = Database::find_all_non_deleted(&db).await;

    match clinical {
        Some(found_clinical) => Ok(Json(found_clinical)),
        None => {
            error!("Didn't find any Clinical data");
            Err(ClinicalError::NoClinicalsFound)
        }
    }
}

#[get("/clinical/{uuid}")]
async fn find_one(
    db: Data<Database>,
    uuid: Path<ClinicalUuid>,
) -> Result<Json<Clinical>, ClinicalError> {
    let clinical_uuid = uuid.into_inner().uuid;
    let clinical_result = Database::find_one(&db, clinical_uuid.clone()).await;

    match clinical_result {
        Some(result) => Ok(Json(result)),
        None => {
            error!("No items found for UUID:: {:?}", &clinical_uuid);
            Err(ClinicalError::NoClinicalsFound)
        }
    }
}

#[post("/clinical")]
async fn create(db: Data<Database>, body: Json<Clinical>) -> Result<Json<Clinical>, ClinicalError> {
    let is_valid = body.validate();
    let date_created = Local::now();
    let mut new_clinical = body.into_inner();
    new_clinical.date_created = Some(date_created.clone());
    new_clinical.date_modified = Some(date_created.clone());
    new_clinical.first_contact_date = Some(date_created.clone());

    match is_valid {
        Ok(_) => {
            let new_uuid = get_uuid();
            let my_clinical =
                Database::add_one(&db, Clinical::new(String::from(new_uuid), new_clinical)).await;

            match my_clinical {
                Some(clinical_result) => Ok(Json(clinical_result)),
                None => {
                    error!("Error [POST] /clinical");
                    Err(ClinicalError::ClinicalCreationFailure)
                }
            }
        }
        Err(e) => {
            error!("Error clinical.create {:?}", e);
            Err(ClinicalError::ClinicalCreationFailure)
        }
    }
}

#[patch("/clinical")]
async fn update_one(
    db: Data<Database>,
    body: Json<Clinical>,
) -> Result<Json<Clinical>, ClinicalError> {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let uuid_id_db = match body.uuid.clone() {
                Some(c_id) => c_id,
                None => String::from(""),
            };
            let stored_clinical = Database::find_one(&db, uuid_id_db).await;
            let date_modified = Local::now();

            let c_cloned = stored_clinical.clone();

            let date_created = match stored_clinical {
                Some(clinical) => clinical.date_created,
                None => {
                    error!("No date found for UUID:: {:?}", body.uuid.clone());
                    Some(Local::now())
                }
            };

            let my_clinical = Clinical {
                uuid: body.uuid.clone(),
                name: body.name.clone(),
                is_company: body.is_company.clone(),
                last_name: body.last_name.clone(),
                clinic_name: match Some(&body.clinic_name) {
                    Some(clinic_name) => clinic_name.clone(),
                    None => None,
                },
                specialty: body.specialty.clone(),
                clinic_web: match Some(&body.clinic_web) {
                    Some(web_url) => web_url.clone(),
                    None => None,
                },
                clinic_email: match Some(&body.clinic_email) {
                    Some(clinic_email) => clinic_email.clone(),
                    None => None,
                },
                email: match Some(&body.email) {
                    Some(email) => email.clone(),
                    None => None,
                },
                phone: body.phone.clone(),
                deleted: body.deleted.clone(),
                fb: match Some(&body.fb) {
                    Some(fb_req) => fb_req.clone(),
                    None => None,
                },
                instagram: match Some(&body.instagram) {
                    Some(ig_req) => ig_req.clone(),
                    None => None,
                },
                linked_in: match Some(&body.linked_in) {
                    Some(li_req) => li_req.clone(),
                    None => None,
                },
                tik_tok: match Some(&body.tik_tok) {
                    Some(tt_req) => tt_req.clone(),
                    None => None,
                },
                twitter: match Some(&body.twitter) {
                    Some(tw_req) => tw_req.clone(),
                    None => None,
                },
                first_contact_date: body.first_contact_date.clone(),
                sales_funnel: body.sales_funnel.clone(),
                notes: body.notes.clone(),
                services_offered: body.services_offered.clone(),
                date_created,
                date_modified: Some(date_modified),
                created_by: match c_cloned {
                    Some(c) => c.created_by,
                    None => None,
                },
                modified_by: match Some(&body.modified_by) {
                    Some(modified_by) => modified_by.clone(),
                    None => Some(String::from("n/a")),
                },
            };

            let updated_clinical = Database::update_one(&db, my_clinical).await;

            match updated_clinical {
                Some(clinical) => Ok(Json(clinical)),
                None => {
                    error!("Error in clinical.update_one");
                    Err(ClinicalError::NoClinicalsFound)
                }
            }
        }
        Err(e) => {
            error!("Error in clinical.update_one: {:?}", e);
            Err(ClinicalError::NoClinicalsFound)
        }
    }
}

#[get("/clinical/deleted")]
async fn find_all_deleted(db: Data<Database>) -> Result<Json<Vec<Clinical>>, ClinicalError> {
    let clinics = Database::find_all_deleted(&db).await;
    match clinics {
        Some(found_clinics) => Ok(Json(found_clinics)),
        None => {
            error!("Didn't find any deleted clinics");
            Err(ClinicalError::NoClinicalsFound)
        }
    }
}

#[delete("/clinical/{uuid}")]
async fn delete_one(
    db: Data<Database>,
    uuid: Path<ClinicalUuid>,
) -> Result<Json<ClinicalUuid>, ClinicalError> {
    let clinic_uuid = uuid.into_inner().uuid;
    let clinic_from_db: Option<Clinical> = Database::delete_one(&db, clinic_uuid.clone()).await;

    match clinic_from_db {
        Some(mut clinic) => {
            clinic.deleted = true;
            match Database::update_one(&db, clinic).await {
                Some(_) => Ok(Json(ClinicalUuid {
                    uuid: clinic_uuid.to_string(),
                })),
                None => {
                    error!("Unable to update clinic :: {:?}", &clinic_uuid);
                    Ok(Json(ClinicalUuid {
                        uuid: "".to_string(),
                    }))
                }
            }
        }
        None => {
            error!("Error [POST] /enterprise");
            Err(ClinicalError::ClinicalCreationFailure)
        }
    }
}

pub fn clinical_api_controllers(cfg: &mut ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find_all_deleted);
    cfg.service(find_one);
    cfg.service(create);
    cfg.service(update_one);
    cfg.service(delete_one);
}
