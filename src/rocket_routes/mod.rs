use diesel::PgConnection;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Status;
use rocket::Response;
use rocket::{response::status::Custom, serde::json::serde_json::json};
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use rocket_db_pools::{deadpool_redis, Connection, Database};

use crate::models::{RoleCode, User};
use crate::repositories::roles::RoleRepository;
use crate::repositories::users::UserRepository;
use rocket::request::{FromRequest, Outcome, Request};

pub mod authorization;
pub mod crates;
pub mod rustaceans;

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(PgConnection);

#[derive(Database)]
#[database("redis")]
pub struct CacheConn(deadpool_redis::Pool);

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Append CORS headers in responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_raw_header("Access-Control-Allow-Origin", "*");
        res.set_raw_header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE");
        res.set_raw_header("Access-Control-Allow-Headers", "*");
        res.set_raw_header("Access-Control-Allow-Credentials", "true");
    }
}

pub struct EditorUser(User);

pub fn server_error(e: Box<dyn std::error::Error>) -> Custom<rocket::serde::json::Value> {
    log::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}

#[allow(clippy::let_unit_value)]
#[rocket::options("/<_..>")]
pub fn options() {
    //Just to add CORS HEADERS via fairing.
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for EditorUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = request
            .guard::<User>()
            .await
            .expect("Can not retrieve logged in user in guard");
        let db = request
            .guard::<DbConn>()
            .await
            .expect("Can not connect to Postgres in request guard");

        let editor_user_options = db
            .run(move |c| {
                let roles_result = RoleRepository::find_by_user(c, &user);

                if let Ok(roles) = roles_result {
                    if roles
                        .iter()
                        .any(|r| r.code == RoleCode::Admin || r.code == RoleCode::Editor)
                    {
                        return Some(EditorUser(user));
                    }
                }
                None
            })
            .await;

        match editor_user_options {
            Some(user) => rocket::outcome::Outcome::Success(user),
            None => rocket::outcome::Outcome::Failure((Status::Forbidden, ())),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let session_header = request
            .headers()
            .get_one("Authorization")
            .map(|v| v.split_whitespace().collect::<Vec<_>>())
            .filter(|v| v.len() == 2 && v[0] == "Bearer")
            .map(|v| v[1]);

        if let Some(session_value) = session_header {
            let mut cache = request
                .guard::<Connection<CacheConn>>()
                .await
                .expect("Can not connect to Redis in request guard");
            let user_id_result = cache
                .get::<_, i32>(format!("sessions/{}", session_value))
                .await;

            if let Ok(user_id) = user_id_result {
                let db = request
                    .guard::<DbConn>()
                    .await
                    .expect("Can not connect to Postgres in request guard");
                let user_result = db.run(move |c| UserRepository::get_by_id(c, user_id)).await;

                if let Ok(user) = user_result {
                    return rocket::outcome::Outcome::Success(user);
                }
            }
        };

        rocket::outcome::Outcome::Failure((Status::Unauthorized, ()))
    }
}
