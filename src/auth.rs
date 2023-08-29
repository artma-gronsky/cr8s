use argon2::{
    password_hash::{Error, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};
use serde::Deserialize;

use crate::models::User;

#[derive(Deserialize)]
pub struct Credetials {
    pub username: String,
    pub password: String,
}

pub fn authrize_user(user: &User, credentials: &Credetials) -> Result<String, Error> {
    let password_hash = PasswordHash::new(&user.password)?;
    let argon = argon2::Argon2::default();
    argon.verify_password(credentials.password.as_bytes(), &password_hash)?;

    Ok(rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect())
}

pub fn hash_password(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(OsRng);
    let argon = Argon2::default();
    argon
        .hash_password(password.as_bytes(), &salt)
        .map(|result| result.to_string())
}
