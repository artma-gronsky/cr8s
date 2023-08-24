use crate::{
    models::{Crate, NewCrate, User},
    repositories::crates::CrateRepository, 
    rocket_routes::{DbConn, server_error},
};
use rocket::{
    http::Status,
    response::status::{Custom, NoContent},
    serde::json::{serde_json::json, Json, Value},
};

// GET crates/
#[rocket::get("/crates")]
pub async fn get_crates(db: DbConn, user: User) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        CrateRepository::find_multiple(c, 100)
            .map(|crates| json!(crates))
            .map_err(|e | server_error(e.into()))
    })
    .await
}

// GET crates/id
#[rocket::get("/crates/<id>")]
pub async fn get_crate(db: DbConn, id: i32, user: User) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::find(c, id)
            .map(|c| json!(c))
            .map_err(|e | server_error(e.into()))
    })
    .await
}

// POST crates/
#[rocket::post("/crates", format = "json", data = "<new>")]
pub async fn create(db: DbConn, new: Json<NewCrate>, user: User) -> Result<Custom<Value>, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::create(c, new.0)
            .map(|c| Custom(Status::Created, json!(c)))
            .map_err(|e | server_error(e.into()))
    })
    .await
}

// PUT crates/id
#[rocket::put("/crates/<id>", format = "json", data = "<crate_for_update>")]
pub async fn update(
    db: DbConn,
    id: i32,
    crate_for_update: Json<Crate>,
    user: User
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::update(c, id, crate_for_update.0)
            .map(|c| json!(c))
            .map_err(|e | server_error(e.into()))
    })
    .await
}

// DELETE crates/id
#[rocket::delete("/crates/<id>")]
pub async fn delete(db: DbConn, id: i32, user: User) -> Result<NoContent, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::delete(c, id)
            .map(|_| NoContent)
            .map_err(|e | server_error(e.into()))
    })
    .await
}
