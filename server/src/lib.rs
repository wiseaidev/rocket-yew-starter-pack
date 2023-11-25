#[macro_use]
extern crate rocket;

mod cors;
mod database;
mod models;
mod routes;

pub use crate::cors::config_cors;
pub use crate::database::setup_database;
pub use crate::models::Task;
pub use crate::routes::all_routes;
