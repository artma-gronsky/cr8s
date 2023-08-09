use rocket::serde::json::Json;

use crate::models::{NewRustacean, Rustacean};

#[rocket::get("/rustaceans")]
pub fn get_rustaceans() {}

#[rocket::get("/rustaceans/<id>")]
pub fn get_rustacean(id: i32) {}

#[rocket::post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub fn create_rustacean(new_rustacean: Json<NewRustacean>) {}

#[rocket::put("/rustaceans/<id>", format = "json", data = "<update_rustacean>")]
pub fn update_rustacean(id: i32, update_rustacean: Json<Rustacean>) {}

#[rocket::delete("/rustaceans/<id>")]
pub fn delete_rustacean(id: i32){}
