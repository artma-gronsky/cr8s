use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::{OsRng};
use diesel::{Connection, PgConnection};
use crate::models::{NewUser, Role, User};
use crate::repositories::roles::RoleRepository;
use crate::repositories::users::UserRepository;

fn get_connection() ->PgConnection{
    let db_url = std::env::var("DATABASE_URL").expect("Cannot load db url from env");
    
    PgConnection::establish(&db_url).expect("Can not connect to postgres")
}

pub fn crate_user(username: String, password: String, role_codes: Vec<String>){
    let salt = SaltString::generate(OsRng);
    let argon = Argon2::default();
    let password_hash = argon.hash_password(password.as_bytes(), &salt).unwrap();
    let new_user = NewUser{username, password: password_hash.to_string()}; 
    let mut c = get_connection();
    
    let user = UserRepository::create(&mut c, new_user, role_codes).unwrap();
    
    println!("User created {:?}", user);
    
    let roles = RoleRepository::find_by_user(&mut c, &user).unwrap();
    for role in roles {
        println!("Role assigned {:?}", role);
    }
} 

pub fn list_users(){
    let mut c = get_connection();
    let users = UserRepository::get_all(&mut c, 100).unwrap();

    println!("User list:");
    for (idx,user) in users.iter().enumerate() {
        println!("{}) {:?}", idx+1, user);
    }
}


pub fn delete_user(id: i32){
    let mut c = get_connection();
    UserRepository::delete(&mut c, id).unwrap();
}