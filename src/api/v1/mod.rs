pub mod auth_controller;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![
        auth_controller::users_login,
        auth_controller::users_register,
    ]
}
