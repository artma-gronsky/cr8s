use diesel::PgConnection;
use rocket::{response::status::Custom, serde::json::serde_json::json, http::Status};
use rocket_db_pools::{deadpool_redis, Database};

pub mod rustaceans;
pub mod crates;
pub mod authorization;

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(PgConnection);

#[derive(Database)]
#[database("redis")]
pub struct CacheConn(deadpool_redis::Pool);

pub fn server_error(e: Box<dyn std::error::Error>) -> Custom<rocket::serde::json::Value> {
    log::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}
    