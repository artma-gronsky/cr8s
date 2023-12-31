extern crate cr8s;
use dotenv::dotenv;
use rocket_db_pools::Database;
use std::env;

#[rocket::main]
async fn main() {
    dotenv().ok();

    print_env_variable();

    let _ = rocket::build()
        .mount(
            "/",
            rocket::routes![
                // options
                cr8s::rocket_routes::options,
                // rustaceans
                cr8s::rocket_routes::rustaceans::get_rustacean,
                cr8s::rocket_routes::rustaceans::get_rustaceans,
                cr8s::rocket_routes::rustaceans::create_rustacean,
                cr8s::rocket_routes::rustaceans::update_rustacean,
                cr8s::rocket_routes::rustaceans::delete_rustacean,
                // crates
                cr8s::rocket_routes::crates::get_crates,
                cr8s::rocket_routes::crates::get_crate,
                cr8s::rocket_routes::crates::create,
                cr8s::rocket_routes::crates::update,
                cr8s::rocket_routes::crates::delete,
                //authorization
                cr8s::rocket_routes::authorization::login,
                cr8s::rocket_routes::authorization::me
            ],
        )
        .attach(cr8s::rocket_routes::DbConn::fairing())
        .attach(cr8s::rocket_routes::Cors)
        .attach(cr8s::rocket_routes::CacheConn::init())
        .launch()
        .await;
}

fn print_env_variable() {
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
}
