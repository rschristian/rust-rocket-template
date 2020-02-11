use crate::config;
use crate::services::auth_service::Auth;

use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub hashed_password: String,
}

#[derive(Serialize)]
pub struct UserAuth<'a> {
    email: &'a str,
    first_name: &'a str,
    last_name: &'a str,
    token: String,
}

impl User {
    pub fn to_user_auth(&self, secret: &[u8]) -> UserAuth {
        let exp = config::token_expire_time();
        let token = Auth {
            id: self.id,
            email: self.email.clone(),
            exp: exp.timestamp(),
        }
            .token(secret);

        UserAuth {
            email: &self.email,
            first_name: &self.first_name,
            last_name: &self.last_name,
            token,
        }
    }
}
