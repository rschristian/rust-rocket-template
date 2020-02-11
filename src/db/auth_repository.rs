use crate::db::Conn;
use crate::models::user::User;
use crate::schema::users;

use crypto::scrypt::{self, ScryptParams};
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub hashed_password: &'a str,
}

pub enum UserCreationError {
    DuplicatedEmail,
}

impl From<Error> for UserCreationError {
    fn from(err: Error) -> UserCreationError {
        if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, info) = &err {
            if let Some("unique_email") = info.constraint_name() { return UserCreationError::DuplicatedEmail }
        }
        panic!("Error creating user: {:?}", err)
    }
}

pub fn register(
    first_name: &str,
    last_name: &str,
    email: &str,
    password: &str,
    conn: Conn,
) -> Result<User, UserCreationError> {
    let hashed_password =
        &scrypt::scrypt_simple(password, &ScryptParams::new(14, 8, 1)).expect("hash error");

    let new_user = &NewUser {
        first_name,
        last_name,
        email,
        hashed_password,
    };

    diesel::insert_into(users::table)
        .values(new_user)
        .get_result::<User>(&*conn)
        .map_err(Into::into)
}

pub fn login(email: &str, password: &str, conn: Conn) -> Option<User> {
    let user = users::table
        .filter(users::email.eq(email))
        .get_result::<User>(&*conn)
        .map_err(|err| eprintln!("login_user: {}", err))
        .ok()?;

    let password_matches = scrypt::scrypt_check(password, &user.hashed_password)
        .map_err(|err| eprintln!("login_user: scrypt_check: {}", err))
        .ok()?;

    if password_matches {
        Some(user)
    } else {
        eprintln!(
            "login attempt for '{}' failed: password doesn't match",
            email
        );
        None
    }
}
