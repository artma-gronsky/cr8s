use diesel::PgConnection;
use dotenv::dotenv;
use std::env;

mod models;
mod repositories;
mod schema;
mod rocket_routes;

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(PgConnection);

#[rocket::main]
async fn main() {

    dotenv().ok();
    
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }


    let _ = rocket::build().mount("/", rocket::routes![
        rocket_routes::rustaceans::get_rustacean,
        rocket_routes::rustaceans::get_rustaceans,
        rocket_routes::rustaceans::create_rustacean,
        rocket_routes::rustaceans::update_rustacean,
        rocket_routes::rustaceans::delete_rustacean
    ])
    .attach(DbConn::fairing())
    .launch().await;
}
