use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{json, Json, Value};

use crate::models::User;
use crate::{
    models::{NewRustacean, Rustacean},
    repositories::rustaceans::RustaceanRepository,
    rocket_routes::{DbConn},
};

//curl 127.0.0.1:8000/rustaceans
#[rocket::get("/rustaceans")]
pub async fn get_rustaceans(db: DbConn, user: User) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        RustaceanRepository::find_multiple(c, 100)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
    })
    .await
}

//curl 127.0.0.1:8000/rustaceans/1     
#[rocket::get("/rustaceans/<id>")]
pub async fn get_rustacean(id: i32, db: DbConn, user: User) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::find(c, id)
            .map(|r| json!(r))
            .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
    })
    .await
}

//curl 127.0.0.1:8000/rustaceans -d '{"name": "Jhon", "email": "jhoe@email.com"}' -H 'Content-type: application/json'       
#[rocket::post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn create_rustacean(
    new_rustacean: Json<NewRustacean>,
    db: DbConn,
    user: User
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::create(c, new_rustacean.0)
            .map(|r| Custom(Status::Created, json!(r)))
            .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
    })
    .await
}

#[rocket::put("/rustaceans/<id>", format = "json", data = "<update_rustacean>")]
pub async fn update_rustacean(
    id: i32,
    update_rustacean: Json<Rustacean>,
    db: DbConn,
    user: User
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::update(c, id, update_rustacean.0)
            .map(|r| json!(r))
            .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
    })
    .await
}

#[rocket::delete("/rustaceans/<id>")]
pub async fn delete_rustacean(id: i32, db: DbConn, user: User) -> Result<NoContent, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::delete(c, id)
            .map(|_| NoContent)
            .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
    })
    .await
}
