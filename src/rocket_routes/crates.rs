use crate::{
    models::{Crate, NewCrate},
    repositories::crate_repository::CrateRepository, rocket_routes::DbConn,
};
use rocket::{
    http::Status,
    response::status::{Custom, NoContent},
    serde::json::{serde_json::json, Json, Value},
};

// GET crates/
#[rocket::get("/crates")]
pub async fn get_crates(db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        CrateRepository::find_multiple(c, 100)
            .map(|crates| json!(crates))
            .map_err(|_| Custom(Status::InternalServerError, json!("error")))
    })
    .await
}

// GET crates/id
#[rocket::get("/crates/<id>")]
pub async fn get_crate(db: DbConn, id: i32) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::find(c, id)
            .map(|c| json!(c))
            .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
    })
    .await
}

// POST crates/
#[rocket::post("/crates", format = "json", data = "<new>")]
pub async fn create(db: DbConn, new: Json<NewCrate>) -> Result<Custom<Value>, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::create(c, new.0)
            .map(|c| Custom(Status::Created, json!(c)))
            .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
    })
    .await
}

// PUT crates/id
#[rocket::put("/crates/<id>", format = "json", data = "<crate_for_update>")]
pub async fn update(
    db: DbConn,
    id: i32,
    crate_for_update: Json<Crate>,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::update(c, id, crate_for_update.0)
            .map(|c| json!(c))
            .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
    })
    .await
}

// DELETE crates/id
#[rocket::delete("/crates/<id>")]
pub async fn delete(db: DbConn, id: i32) -> Result<NoContent, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::delete(c, id)
            .map(|_| NoContent)
            .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
    })
    .await
}
