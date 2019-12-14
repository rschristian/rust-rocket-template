#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate validator_derive;

mod api;
mod config;
mod db;
mod errors;
mod models;
mod schema;
mod services;

use rocket::{Rocket, Route};
use rocket_contrib::json::JsonValue;
use validator;

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn rocket_instance(mounts: Vec<(&str, Vec<Route>)>) -> Rocket {
    let mut instance = rocket::ignite();

    for (path, methods) in mounts {
        instance = instance.mount(path, methods);
    }

    instance
        .attach(db::Conn::fairing())
        .register(catchers![not_found])
}

fn mounts() -> Vec<(&'static str, Vec<Route>)> {
    vec![("/api/v1", api::v1::routes())]
}

pub fn rocket() -> rocket::Rocket {
    rocket_instance(mounts())
}
