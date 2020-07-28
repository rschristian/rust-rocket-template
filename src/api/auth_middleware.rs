use crate::config;

use jsonwebtoken as jwt;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::{Outcome, State};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    pub id: i32,
    pub email: String,
    pub exp: i64,
}

impl Auth {
    pub fn token(&self, secret: &[u8]) -> String {
        jwt::encode(
            &jwt::Header::default(),
            self,
            &jwt::EncodingKey::from_secret(secret),
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
        let state: State<config::AppState> = request.guard()?;
        if let Some(auth) = extract_auth_from_request(request, &state.secret) {
            Outcome::Success(auth)
        } else {
            Outcome::Failure((Status::Forbidden, ()))
        }
    }
}

fn extract_auth_from_request(request: &Request, secret: &[u8]) -> Option<Auth> {
    request
        .headers()
        .get_one("authorization")
        .and_then(extract_token_from_header)
        .and_then(|token| decode_token(token, secret))
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
fn decode_token(token: &str, secret: &[u8]) -> Option<Auth> {
    jwt::decode(
        token,
        &jwt::DecodingKey::from_secret(secret),
        &jwt::Validation::new(jwt::Algorithm::HS256),
    )
    .map_err(|err| {
        eprintln!("Auth decode error: {:?}", err);
    })
    .ok()
    .map(|token_data| token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    const JWT_TOKEN: &str =
        "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJlbWFpbCI6InNtb2tldGVzdEBleGFtcGxlL\
        mNvbSIsImV4cCI6MTY4NjAyNzUxNCwiaWQiOjF9.DB-Mnal2QvBZArsXUY6SXaS_hq_sefXtMHECdM70W3M";

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

    // TODO: Move to integrations tests, can't test token here if the secret is handled by Rocket
    //    #[test]
    //    fn test_decode_token() {
    //        let auth: Auth = decode_token(JWT_TOKEN).unwrap();
    //
    //        assert_eq!(2, auth.id);
    //        assert_eq!("ryan@gmail.com", auth.email);
    //    }
    //
    //    #[test]
    //    fn test_decode_token_bad_signature() {
    //        const JWT_TOKEN_BAD_SIGNATURE: &'static str =
    //            "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJlbWFpbCI6InNtb2tldGVzdEBleGFtcGxlL\
    //            mNvbSIsImV4cCI6MTY4NjAyNzUxNCwiaWQiOjF9.6k4orw9QbAJ9r8Si8LwIQwfz15Efx2QPH2mvMHrDq8E";
    //
    //        let auth: Option<Auth> = decode_token(JWT_TOKEN_BAD_SIGNATURE);
    //        assert!(auth.is_none());
    //    }
}
