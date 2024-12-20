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
    db::{config::Database, school_db::SchoolDB},
    error::school_error::SchoolError,
    models::school_model::{School, SchoolUuid},
    utils::{
        general_utils::{get_uuid, shuffle_id},
        message_utils::get_validation_errors,
    },
};

#[get("/schools")]
#[tracing::instrument(name = "Find all schools", skip(db))]
async fn find_all(db: Data<Database>) -> Result<HttpResponse, SchoolError> {
    let school = Database::find_all_active(&db).await;

    match school {
        Some(schools_found) => Ok(HttpResponse::Ok()
            .status(StatusCode::OK)
            .json(schools_found)),
        None => {
            error!("Didn't find any School data");
            Ok(HttpResponse::NotFound().json(SchoolUuid {
                uuid: format!("{}", SchoolError::NoSchoolsFound),
            }))
        }
    }
}

#[get("/schools/{uuid}")]
#[tracing::instrument(name = "Find one school", skip(db), fields(uuid = %uuid.uuid))]
async fn find_one(db: Data<Database>, uuid: Path<SchoolUuid>) -> Result<HttpResponse, SchoolError> {
    let school_uuid = uuid.into_inner().uuid;
    let school_result = Database::find_one(&db, school_uuid.clone()).await;

    match school_result {
        Some(result) => Ok(HttpResponse::Ok().status(StatusCode::OK).json(result)),
        None => {
            error!("No schools found for id:: {:?}", &school_uuid);
            Ok(HttpResponse::NotFound().json(SchoolUuid {
                uuid: format!("{}", SchoolError::NoSchoolsFound),
            }))
        }
    }
}

#[post("/schools")]
#[tracing::instrument(
    name = "Find one school",
    skip(db),
    fields(
        name = %body.name,
        last_name = %body.last_name,
        school_name = %body.school_name,
    )
)]
async fn create(db: Data<Database>, body: Json<School>) -> Result<HttpResponse, SchoolError> {
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
                Some(school_result) => Ok(HttpResponse::Ok()
                    .insert_header(("HX-Trigger", "school_reload_page"))
                    .status(StatusCode::CREATED)
                    .json(SchoolUuid {
                        uuid: match school_result.uuid {
                            Some(school_uuid) => school_uuid,
                            None => "".to_string(),
                        },
                    })),
                None => {
                    error!("Error [POST] /school");
                    Ok(HttpResponse::InternalServerError()
                        .insert_header((
                            "HX-Trigger",
                            format!("{{ \"page_error\": \"Internal server error\" }}"),
                        ))
                        .json(SchoolUuid {
                            uuid: "Error".to_string(),
                        }))
                }
            }
        }
        Err(e) => {
            error!("Error School.create {:?}", &e);
            let key_errors_vec: Vec<String> = get_validation_errors(&e);

            Ok(HttpResponse::InternalServerError()
                .insert_header((
                    "HX-Trigger",
                    format!("{{ \"page_error\": {:?} }}", key_errors_vec),
                ))
                .json(SchoolUuid {
                    uuid: format!("{}", SchoolError::SchoolCreationFailure),
                }))
        }
    }
}

#[patch("/schools")]
#[tracing::instrument(name = "Patch School", skip(db), fields(
    name = %body.name,
    last_name = %body.last_name,
    school_name = %body.school_name,
))]
async fn update_one(db: Data<Database>, body: Json<School>) -> Result<HttpResponse, SchoolError> {
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
                school_web: match Some(&body.school_web) {
                    Some(school_page) => school_page.clone(),
                    None => None,
                },
                school_email: match Some(&body.school_email) {
                    Some(school_email) => school_email.clone(),
                    None => None,
                },
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
                Some(school) => Ok(HttpResponse::Ok()
                    .insert_header(("HX-Trigger", "school_reload_page"))
                    .status(StatusCode::OK)
                    .json(SchoolUuid {
                        uuid: match school.uuid {
                            Some(this_uuid) => shuffle_id(this_uuid),
                            None => "".to_string(),
                        },
                    })),
                None => {
                    error!("Error updating school");

                    Ok(HttpResponse::InternalServerError()
                        .insert_header((
                            "HX-Trigger",
                            format!(
                                "{{ \"page_error\": {:?} }}",
                                "Couldn't find school".to_string()
                            ),
                        ))
                        .json(SchoolUuid {
                            uuid: format!("{}", SchoolError::NoSchoolsFound),
                        }))
                }
            }
        }
        Err(e) => {
            error!("Error School.update_one {:?}", &e);
            let key_errors_vec: Vec<String> = get_validation_errors(&e);

            Ok(HttpResponse::InternalServerError()
                .insert_header((
                    "HX-Trigger",
                    format!("{{ \"page_error\": {:?} }}", key_errors_vec),
                ))
                .json(SchoolUuid {
                    uuid: format!("{}", SchoolError::SchoolCreationFailure),
                }))
        }
    }
}

#[get("/schools/deleted")]
#[tracing::instrument(name = "Show Deleted Schools", skip(db))]
async fn find_all_deleted(db: Data<Database>) -> Result<HttpResponse, SchoolError> {
    let schools = Database::find_all_deleted(&db).await;

    match schools {
        Some(deleted_schools) => Ok(HttpResponse::Ok()
            .insert_header(("HX-Trigger", "school_fad"))
            .status(StatusCode::OK)
            .json(deleted_schools)),
        None => {
            error!("Didn't find any deleted schools");
            Ok(HttpResponse::NotFound().json(SchoolUuid {
                uuid: "Error".to_string(),
            }))
        }
    }
}

#[delete("/schools/{uuid}")]
#[tracing::instrument(name = "Delete School", skip(db), fields(uuid = %uuid.uuid))]
async fn delete_one(
    db: Data<Database>,
    uuid: Path<SchoolUuid>,
) -> Result<HttpResponse, SchoolError> {
    let school_uuid = uuid.into_inner().uuid;
    let school_from_db: Option<School> = Database::delete_one(&db, school_uuid.clone()).await;

    match school_from_db {
        Some(mut school) => {
            school.deleted = true;
            match Database::update_one(&db, school).await {
                Some(_) => Ok(HttpResponse::Ok()
                    .insert_header(("HX-Trigger", "school_delete"))
                    .status(StatusCode::OK)
                    .json(SchoolUuid {
                        uuid: school_uuid.to_string(),
                    })),
                None => {
                    error!("unable to delete school:: {:?}", &school_uuid);
                    Ok(HttpResponse::InternalServerError().json(SchoolUuid {
                        uuid: format!("{}", SchoolError::NoSchoolsFound),
                    }))
                }
            }
        }
        None => {
            error!("Unable to update school :: {:?}", &school_uuid);
            Ok(HttpResponse::NotFound().json(SchoolUuid {
                uuid: format!("{}", SchoolError::NoSchoolsFound),
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
