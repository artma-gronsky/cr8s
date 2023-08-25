use diesel::PgConnection;
use rocket::http::Status;
use rocket::{response::status::Custom, serde::json::serde_json::json};
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use rocket_db_pools::{deadpool_redis, Database, Connection};

use crate::models::User;
use crate::repositories::users::UserRepository;
use rocket::request::{Outcome, Request, FromRequest};

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
    

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self,Self::Error> {
       let session_header = request.headers().get_one("Authorization")
       .map(|v| v.split_whitespace().collect::<Vec<_>>())
       .filter(|v| v.len() == 2 && v[0] == "Bearer")
       .map(|v| v[1]);
        

        if let Some(session_value) = session_header{
            let mut cache = request.guard::<Connection<CacheConn>>().await.expect("Can not connect to Redis in request guard");
            let user_id_result = cache.get::<_, i32>(format!("sessions/{}", session_value)).await;

            if let Ok(user_id)  = user_id_result{
                let db = request.guard::<DbConn>().await.expect("Can not connect to Postgres in request guard");
                let user_result = db.run(move |c|{
                    UserRepository::get_by_id(c, user_id)
                }).await;  
    
                if let Ok(user) = user_result{
                    return rocket::outcome::Outcome::Success(user);
                }
            }
        };

        rocket::outcome::Outcome::Failure((Status::Unauthorized, ()))
    }
}