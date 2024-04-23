use argon2::{
    password_hash::{rand_core::OsRng, Error, PasswordHasher, SaltString},
    Argon2,
};
// use chrono::Local;
// use log::info;

// use crate::models::users_model::User;

pub fn pwd_hasher(my_password: String) -> Result<String, Error> {
    let password: &[u8] = my_password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password, &salt)?.to_string();

    // Return the owned string directly instead of a reference to it
    Ok(password_hash)
}

// pub fn hash_user_password(
//     cloned_pwd_from_json: Option<String>,
//     user_in_db: Option<User>,
// ) -> String {
//     let mut final_password = "".to_string();
//     info!("{}", &final_password);

//     match cloned_pwd_from_json {
//         Some(pwd) => match pwd_hasher(pwd) {
//             Ok(new_pwd) => {
//                 final_password = new_pwd.clone();
//             }
//             Err(_) => {
//                 final_password = format!("default_pwd_created_at_{:?}", Local::now());
//             }
//         },
//         None => match &user_in_db {
//             Some(u_db) => match &u_db.password {
//                 Some(old_pwd) => {
//                     final_password = old_pwd.to_string();
//                 }
//                 None => {
//                     final_password = format!("default_pwd_created_at_{:?}", Local::now());
//                 }
//             },
//             None => {
//                 final_password = format!("default_pwd_created_at_{:?}", Local::now());
//             }
//         },
//     }

//     final_password
// }
