use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash,
        PasswordHasher,
        PasswordVerifier,
        SaltString,
    },
    Argon2,
    Algorithm,
    Version,
    Params,
};

use crate::errors::ErrorMessage;

const MAX_PASSWORD_LENGTH: usize = 64;

pub fn hash(password: impl Into<String>) -> Result<String, ErrorMessage> {
    let password = password.into();

    if password.is_empty(){
        return Err(ErrorMessage::EmptyPassword);
    }

    if password.len() > MAX_PASSWORD_LENGTH {
        return Err(ErrorMessage::ExceededMaxPasswordLength(MAX_PASSWORD_LENGTH));
    }

    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::new(
        Algorithm::Argon2id, //Balance between security and performance
        Version::V0x13,
        Params::new(
            15*1024, // Memory cost
            2, // Time cost
            1, // Threads
            None, //Optional output length
        ).map_err(|_| ErrorMessage::HashingError)?,
    );

    let hashed_password = argon2
    .hash_password(password.as_bytes(), &salt)
    .map_err(|_| ErrorMessage::HashingError)?
    .to_string();

    Ok(hashed_password)

}

pub fn compare(password: &str, hashed_password: &str) -> Result<bool , ErrorMessage> {
    if password.is_empty(){
        return Err(ErrorMessage::EmptyPassword);
    }

    if password.len() > MAX_PASSWORD_LENGTH{
        return Err(ErrorMessage::ExceededMaxPasswordLength(MAX_PASSWORD_LENGTH));
    }

    let parsed_hash = PasswordHash::new(hashed_password).map_err(|_| ErrorMessage::InvalidHashFormat)?;

    let password_matched = Argon2::default().verify_password(password.as_bytes() , &parsed_hash).map_or(false, |_| true);

    Ok(password_matched)
}


