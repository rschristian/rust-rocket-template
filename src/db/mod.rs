use rocket_contrib::databases::diesel;

pub mod auth_repository;

#[database("diesel_postgres_pool")]
pub struct Conn(diesel::PgConnection);
