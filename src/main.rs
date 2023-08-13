use dotenv::dotenv;
use std::env;

mod models;
mod repositories;
mod rocket_routes;
mod schema;

#[rocket::main]
async fn main() {
    dotenv().ok();

    print_env_variable();

    let _ = rocket::build()
        .mount(
            "/",
            rocket::routes![
                // rustaceans
                rocket_routes::rustaceans::get_rustacean,
                rocket_routes::rustaceans::get_rustaceans,
                rocket_routes::rustaceans::create_rustacean,
                rocket_routes::rustaceans::update_rustacean,
                rocket_routes::rustaceans::delete_rustacean,
                // crates
                rocket_routes::crates::get_crates,
                rocket_routes::crates::get_crate,
                rocket_routes::crates::create,
                rocket_routes::crates::update,
                rocket_routes::crates::delete,
            ],
        )
        .attach(crate::rocket_routes::DbConn::fairing())
        .launch()
        .await;
}

fn print_env_variable() {
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
}
