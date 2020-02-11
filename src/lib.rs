#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate validator_derive;

extern crate dotenv;

mod api;
mod config;
mod db;
mod errors;
mod models;
mod schema;
mod services;

use dotenv::dotenv;
use rocket::{Rocket, Route};
use rocket_contrib::json::JsonValue;

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn rocket_instance(mounts: Vec<(&str, Vec<Route>)>) -> Rocket {
    let mut instance = rocket::custom(config::from_env());

    for (path, methods) in mounts {
        instance = instance.mount(path, methods);
    }

    instance
        .attach(db::Conn::fairing())
        .attach(config::AppState::manage())
        .register(catchers![not_found])
}

fn mounts() -> Vec<(&'static str, Vec<Route>)> {
    vec![("/api/v1", api::v1::routes())]
}

pub fn rocket() -> rocket::Rocket {
    dotenv().ok();
    rocket_instance(mounts())
}
