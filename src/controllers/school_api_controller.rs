use actix_web::{
    delete, get, patch, post,
    web::{Data, Json, Path, ServiceConfig},
};
use chrono::Local;
use log::error;
use validator::Validate;

use crate::{
    db::{config::Database, school_db::SchoolDB},
    error::school_error::SchoolError,
    models::school_model::{School, SchoolUuid},
    utils::general_utils::get_uuid,
};

#[get("/schools")]
async fn find_all(db: Data<Database>) -> Result<Json<Vec<School>>, SchoolError> {
    let school = Database::find_all_active(&db).await;

    match school {
        Some(schools_found) => Ok(Json(schools_found)),
        None => {
            error!("Didn't find any School data");
            Err(SchoolError::NoSchoolsFound)
        }
    }
}

#[get("/schools/{uuid}")]
async fn find_one(db: Data<Database>, uuid: Path<SchoolUuid>) -> Result<Json<School>, SchoolError> {
    let school_uuid = uuid.into_inner().uuid;
    let school_result = Database::find_one(&db, school_uuid.clone()).await;

    match school_result {
        Some(result) => Ok(Json(result)),
        None => {
            error!("No schools found for id:: {:?}", &school_uuid);
            Err(SchoolError::NoSchoolsFound)
        }
    }
}

#[post("/schools")]
async fn create(db: Data<Database>, body: Json<School>) -> Result<Json<SchoolUuid>, SchoolError> {
    let is_valid = body.validate();
    let date_created = Local::now();

    let mut new_school = body.into_inner();

    new_school.date_created = Some(date_created.clone());
    new_school.date_modified = Some(date_created.clone());
    new_school.first_contact_date = Some(date_created.clone());

    match is_valid {
        Ok(_) => {
            let new_uuid = get_uuid();
            let my_school =
                Database::add_one(&db, School::new(String::from(new_uuid), new_school)).await;

            match my_school {
                Some(school_result) => Ok(Json(SchoolUuid {
                    uuid: match school_result.uuid {
                        Some(school_uuid) => school_uuid,
                        None => "".to_string(),
                    },
                })),
                None => {
                    error!("Error [POST] /school");
                    Err(SchoolError::SchoolCreationFailure)
                }
            }
        }
        Err(e) => {
            error!("Error School.create {:?}", e);
            Err(SchoolError::SchoolCreationFailure)
        }
    }
}

#[patch("/schools")]
async fn update_one(db: Data<Database>, body: Json<School>) -> Result<Json<School>, SchoolError> {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let uuid_id_db = match body.uuid.clone() {
                Some(school_id) => school_id,
                None => String::from(""),
            };
            let stored_school = Database::find_one(&db, uuid_id_db).await;
            let date_modified = Local::now();

            let school_cloned = stored_school.clone();

            let date_created = match stored_school {
                Some(school) => school.date_created,
                None => {
                    error!("No date found for uuid:: {:?}", body.uuid.clone());
                    Some(Local::now())
                }
            };

            let my_school = School {
                uuid: body.uuid.clone(),
                name: body.name.clone(),
                last_name: body.last_name.clone(),
                school_name: body.school_name.clone(),
                school_level: body.school_level.clone(),
                email: body.email.clone(),
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
                created_by: match school_cloned {
                    Some(s) => s.created_by,
                    None => None,
                },
                modified_by: match Some(&body.modified_by) {
                    Some(modified_by) => modified_by.clone(),
                    None => Some(String::from("n/a")),
                },
            };

            let updated_school = Database::update_one(&db, my_school).await;

            match updated_school {
                Some(school) => Ok(Json(school)),
                None => {
                    error!("Error updating school");
                    Err(SchoolError::NoSchoolsFound)
                }
            }
        }
        Err(e) => {
            error!("Error in school.update_one: {:?}", e);
            Err(SchoolError::NoSchoolsFound)
        }
    }
}

#[get("/schools/deleted")]
async fn find_all_deleted(db: Data<Database>) -> Result<Json<Vec<School>>, SchoolError> {
    let schools = Database::find_all_deleted(&db).await;

    match schools {
        Some(deleted_schools) => Ok(Json(deleted_schools)),
        None => {
            error!("Didn't find any deleted schools");
            Err(SchoolError::NoSchoolsFound)
        }
    }
}

#[delete("/schools/{uuid}")]
async fn delete_one(
    db: Data<Database>,
    uuid: Path<SchoolUuid>,
) -> Result<Json<SchoolUuid>, SchoolError> {
    let school_uuid = uuid.into_inner().uuid;
    let school_from_db: Option<School> = Database::delete_one(&db, school_uuid.clone()).await;

    match school_from_db {
        Some(mut school) => {
            school.deleted = true;
            match Database::update_one(&db, school).await {
                Some(_) => Ok(Json(SchoolUuid {
                    uuid: school_uuid.to_string(),
                })),
                None => {
                    error!("unable to delete school:: {:?}", &school_uuid);
                    Ok(Json(SchoolUuid {
                        uuid: "".to_string(),
                    }))
                }
            }
        }
        None => {
            error!("Unable to update school :: {:?}", &school_uuid);
            Ok(Json(SchoolUuid {
                uuid: "".to_string(),
            }))
        }
    }
}

pub fn school_api_controllers(cfg: &mut ServiceConfig) {
    cfg.service(create);
    cfg.service(delete_one);
    cfg.service(find_all);
    cfg.service(find_all_deleted);
    cfg.service(find_one);
    cfg.service(update_one);
}
