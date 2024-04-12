use argon2::{
    password_hash::{rand_core::OsRng, Error, PasswordHasher, SaltString},
    Argon2,
};

pub fn pwd_hasher(my_password: String) -> Result<String, Error> {
    let password: &[u8] = my_password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password, &salt)?.to_string();

    // Return the owned string directly instead of a reference to it
    Ok(password_hash)
}
