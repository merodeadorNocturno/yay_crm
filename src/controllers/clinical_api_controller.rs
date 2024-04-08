use actix_web::{
    get, patch, post,
    web::{Data, Json, Path, ServiceConfig},
};

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
    let clinical = Database::find_all(&db).await;

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
    let new_clinical = body.into_inner();

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
            let my_clinical = Clinical {
                uuid: body.uuid.clone(),
                name: body.name.clone(),
                is_company: body.is_company.clone(),
                last_name: body.last_name.clone(),
                specialty: body.specialty.clone(),
                email: body.email.clone(),
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
                resolution: match Some(&body.resolution) {
                    Some(resolution_req) => resolution_req.clone(),
                    None => None,
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

pub fn clinical_api_controllers(cfg: &mut ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find_one);
    cfg.service(create);
    cfg.service(update_one);
}
