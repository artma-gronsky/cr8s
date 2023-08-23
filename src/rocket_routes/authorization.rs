use rocket::{serde::json::{Json, serde_json::json, Value}, response::status::Custom};

use crate::{repositories::users::UserRepository, auth::{Credetials, self}};
use rocket_db_pools::Connection;

use super::{DbConn, CacheConn, server_error};


//docker-compose exec app curl http://127.0.0.1:8000/login -d '{"username": "artmadar", "password":"123"}' -H 'Content-type: application/json'
#[rocket::post("/login", format="json", data="<credatials>")]
pub async fn login(credatials: Json<Credetials>, db: DbConn, cache: Connection<CacheConn>) -> Result<Value, Custom<Value>>{
    db.run(move |c| {
        UserRepository::get_by_name(c, &credatials.username)
        .map(|u|
            match auth::authrize_user(&u, &credatials) {
                Ok(token) =>  json!(token),
                Err(_) => json!("Unauthorized")
        })
        .map_err(|e| server_error(e.into()))
    }).await
}