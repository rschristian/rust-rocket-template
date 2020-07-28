use crate::db::{
    auth_repository::{self, UserCreationError},
    Conn,
};
use crate::models::user::{InsertableUser, User, UserCredentials};

use crypto::scrypt::{self, ScryptParams};

pub fn register(mut user: InsertableUser, conn: Conn) -> Result<User, UserCreationError> {
    user.password = scrypt::scrypt_simple(user.password.as_ref(), &ScryptParams::new(14, 8, 1))
        .expect("hash error");
    auth_repository::register(user, conn)
}

pub fn login(credentials: UserCredentials, conn: Conn) -> Option<User> {
    let user = auth_repository::login(credentials.email, conn);
    match user {
        Some(user) => {
            let password_matches =
                scrypt::scrypt_check(credentials.password.as_ref(), user.hashed_password.as_ref())
                    .map_err(|err| eprintln!("login_user: scrypt_check: {}", err))
                    .ok()?;

            if password_matches {
                Some(user)
            } else {
                None
            }
        }
        None => None,
    }
}
