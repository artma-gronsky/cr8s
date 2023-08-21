use argon2::{PasswordHash, PasswordVerifier, password_hash::Error};
use rand::{Rng, distributions::Alphanumeric};
use serde::Deserialize;

use crate::models::User;

#[derive(Deserialize)]
pub struct Credetials{
    pub username: String, 
    pub password:String
} 

pub fn authrize_user(user: &User, credentials: &Credetials) -> Result<String, Error>{
    
    let password_hash = PasswordHash::new(&user.password)?;
    let argon = argon2::Argon2::default();
    argon.verify_password(credentials.password.as_bytes(), &password_hash)?;

    
    Ok(
        rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128).map(char::from).collect()
    )
}