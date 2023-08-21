use diesel::PgConnection;
use rocket::{response::status::Custom, serde::json::serde_json::json, http::Status};

pub mod rustaceans;
pub mod crates;
pub mod authorization;

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(PgConnection);

pub fn server_error(e: Box<dyn std::error::Error>) -> Custom<rocket::serde::json::Value> {
    log::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}
    