use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{serde_json::json, Json, Value},
};

use crate::{
    auth::{self, Credetials},
    repositories::users::UserRepository, models::User,
};
use rocket_db_pools::{deadpool_redis::redis::AsyncCommands, Connection};

use super::{server_error, CacheConn, DbConn};

//docker-compose exec app curl http://127.0.0.1:8000/login -d '{"username": "artmadar", "password":"123"}' -H 'Content-type: application/json'
#[rocket::post("/login", format = "json", data = "<credatials>")]
pub async fn login(
    credatials: Json<Credetials>,
    db: DbConn,
    mut cache: Connection<CacheConn>,
) -> Result<Value, Custom<Value>> {
    let username = credatials.username.to_owned();
    let user = db
        .run(move |c| UserRepository::get_by_name(c, &username).map_err(|e| 
            {
                match e.to_string() == "Record not found" {
                    true => Custom(Status::Unauthorized, json!("Wrong credentials")),
                    false => server_error(e.into())
                }
            }
        ))
        .await?;

    let session_id = auth::authrize_user(&user, &credatials)
        .map_err(|_err| Custom(Status::Unauthorized, json!("Wrong credentials")))?;

    cache
        .set_ex::<_,_,()>(format!("sessions/{}", session_id), user.id, 3 * 60 * 60)
        .await
        .map(|_| json!({"token": session_id}))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/me")]
pub async fn me(
    user: User
) -> Result<Value, Custom<Value>> {
    Ok(json!(user))
}


