use crate::config;
use crate::db::{
    auth_repository::{self, UserCreationError},
    Conn,
};
use crate::models::user::User;

use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use serde::{Deserialize, Serialize};

use frank_jwt as jwt;
use serde_json;

pub fn register(
    first_name: &str,
    last_name: &str,
    email: &str,
    password: &str,
    conn: Conn,
) -> Result<User, UserCreationError> {
    auth_repository::register(&first_name, &last_name, &email, &password, conn)
}

pub fn login(email: &str, password: &str, conn: Conn) -> Option<User> {
    auth_repository::login(&email, &password, conn)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    pub id: i32,
    pub email: String,
    pub exp: i64,
}

impl Auth {
    pub fn token(&self) -> String {
        let headers = json!({});
        let payload = json!(self);
        jwt::encode(
            headers.0,
            &config::SECRET.to_string(),
            &payload,
            jwt::Algorithm::HS256,
        )
        .expect("jwt")
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = ();

    /// Extract Auth token from the "Authorization" header.
    ///
    /// Handlers with Auth guard will fail with 503 error.
    /// Handlers with Option<Auth> will be called with None.
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Auth, Self::Error> {
        if let Some(auth) = extract_auth_from_request(request) {
            Outcome::Success(auth)
        } else {
            Outcome::Failure((Status::Forbidden, ()))
        }
    }
}

fn extract_auth_from_request(request: &Request) -> Option<Auth> {
    request
        .headers()
        .get_one("authorization")
        .and_then(extract_token_from_header)
        .and_then(decode_token)
}

fn extract_token_from_header(header: &str) -> Option<&str> {
    if header.starts_with(config::TOKEN_PREFIX) {
        Some(&header[config::TOKEN_PREFIX.len()..])
    } else {
        None
    }
}

/// Decode token into `Auth` struct. If any error is encountered, log it
/// and return None.
fn decode_token(token: &str) -> Option<Auth> {
    jwt::decode(token, &config::SECRET.to_string(), jwt::Algorithm::HS256)
        .map(|(_, payload)| {
            serde_json::from_value::<Auth>(payload)
                .map_err(|err| {
                    eprintln!("Auth serde decode error: {:?}", err);
                })
                .ok()
        })
        .unwrap_or_else(|err| {
            eprintln!("Auth decode error: {:?}", err);
            None
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const JWT_TOKEN: &'static str =
        "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJlbWFpbCI6InJ5YW5AZ21haWwuY29tIiwiZXhwIjoxNTczM\
         Tg1ODQ1LCJpZCI6Mn0.IW8XrGMiPh0ZKTs8vICMgANP0-HuRbzd0JnW8H0y-Bg";

    #[test]
    fn test_extract_token() {
        let mut auth_header: String = "Bearer ".to_owned();
        auth_header.push_str(JWT_TOKEN);

        let token: &str = extract_token_from_header(&auth_header).unwrap();
        assert_eq!(JWT_TOKEN, token);
    }

    #[test]
    fn test_extract_token_bad_prefix() {
        let mut bad_auth_header: String = "Bad prefix ".to_owned();
        bad_auth_header.push_str(JWT_TOKEN);

        let token: Option<&str> = extract_token_from_header(&bad_auth_header);
        assert!(token.is_none());
    }

    #[test]
    fn test_decode_token() {
        let auth: Auth = decode_token(JWT_TOKEN).unwrap();

        assert_eq!(2, auth.id);
        assert_eq!("ryan@gmail.com", auth.email);
    }

    #[test]
    fn test_decode_token_bad_signature() {
        const JWT_TOKEN_BAD_SIGNATURE: &'static str =
            "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJlbWFpbCI6InJ5YW5AZ21haWwuY29tIiw\
             iZXhwIjoxNTczMTg1ODQ1LCJpZCI6Mn0.704WFHE8VY0_WLDfKRfYrKvXWARcgGEIieolR7TkAYU";

        let auth: Option<Auth> = decode_token(JWT_TOKEN_BAD_SIGNATURE);
        assert!(auth.is_none());
    }
}
