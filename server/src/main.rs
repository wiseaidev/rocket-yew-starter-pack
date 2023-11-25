#[macro_use]
extern crate rocket;

mod cors;
mod database;
mod models;
mod routes;

use crate::cors::config_cors;
use crate::database::setup_database;
use crate::routes::all_routes;
use rocket::http::Method::{Delete, Get, Options, Post, Put};
use rocket_cors::AllowedOrigins;
use shuttle_rocket;

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let allowed_origins = AllowedOrigins::all();
    let allowed_methods = vec![Get, Post, Options, Put, Delete];
    let cors = config_cors(allowed_origins, allowed_methods);
    let path = "data.db".into();
    let db_arc = setup_database(path);
    let routes = all_routes();
    let rocket_app = rocket::build()
        .mount("/", routes)
        .attach(cors)
        .manage(db_arc);

    Ok(rocket_app.into())
}
