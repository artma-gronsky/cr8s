use std::str::FromStr;

use diesel::{Connection, PgConnection};
use crate::auth;
use crate::models::{NewUser, RoleCode};
use crate::repositories::roles::RoleRepository;
use crate::repositories::users::UserRepository;

fn get_connection() ->PgConnection{
    let db_url = std::env::var("DATABASE_URL").expect("Cannot load db url from env");
    
    PgConnection::establish(&db_url).expect("Can not connect to postgres")
}

pub fn crate_user(username: String, password: String, role_codes: Vec<String>){
    let password_hash = auth::hash_password(&password).unwrap();

    let new_user = NewUser{username, password: password_hash}; 
    let mut c = get_connection();
    

    let role_codes = role_codes.iter().map(|v| RoleCode::from_str(v).unwrap_or(Default::default())).collect();
    let user = UserRepository::create(&mut c, new_user, role_codes).unwrap();
    
    println!("User created {:?}", user);
    
    let roles = RoleRepository::find_by_user(&mut c, &user).unwrap();
    for role in roles {
        println!("Role assigned {:?}", role);
    }
} 

pub fn list_users(){
    let mut c = get_connection();
    let users = UserRepository::find_all_with_roles(&mut c).unwrap();

    println!("User list:");
    for (idx,user) in users.iter().enumerate() {
        println!("{}) {:?}", idx+1, user);
    }
}


pub fn delete_user(id: i32){
    let mut c = get_connection();
    UserRepository::delete(&mut c, id).unwrap();
}