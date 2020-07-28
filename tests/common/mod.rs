#![allow(unused)]

//! This file contains utility functions used by all tests.

use once_cell::sync::OnceCell;
use rocket::http::{ContentType, Header, Status};
use rocket::local::{Client, LocalResponse};
use rust_rocket_template;
use serde_json::Value;

pub const FIRST_NAME: &'static str = "smoke";
pub const LAST_NAME: &'static str = "test";
pub const EMAIL: &'static str = "smoketest@example.com";
pub const PASSWORD: &'static str = "qweasdzxc";

/// Utility macro for turning `json!` into string.
#[macro_export]
macro_rules! json_string {
    ($value:tt) => {
        serde_json::to_string(&serde_json::json!($value)).expect("cannot json stringify")
    };
}

pub type Token = String;

pub fn test_client() -> &'static Client {
    static INSTANCE: OnceCell<Client> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let rocket = rust_rocket_template::rocket();
        Client::new(rocket).expect("valid rocket instance")
    })
}

/// Retrieve a token registering a user if required.
pub fn login(client: &Client) -> Token {
    try_login(client).unwrap_or_else(|| {
        register(client, FIRST_NAME, LAST_NAME, EMAIL, PASSWORD);
        try_login(client).expect("Cannot login")
    })
}

/// Valid token of user that does not yet exist
pub fn get_valid_token_of_invalid_user() -> Token {
    "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJlbWFpbCI6InNtb2tldGVzdDJAZ21haWwuY29tIiwiZXhwI\
     joxNTc1OTA1MjQ2LCJpZCI6NTB9.PbjzU5NvKPxh_vbGADc-eTT23Aqov9Z-keQbxqIRr7E"
        .parse()
        .unwrap()
}

/// Make an authorization header.
pub fn token_header(token: Token) -> Header<'static> {
    Header::new("authorization", format!("Bearer {}", token))
}

/// Helper function for converting response to json value.
pub fn response_json_value(response: &mut LocalResponse) -> Value {
    let body = response.body().expect("no body");
    serde_json::from_reader(body.into_inner()).expect("can't parse value")
}

// Internal stuff

/// Login as default user returning None if login is not found
fn try_login(client: &Client) -> Option<Token> {
    let response = &mut client
        .post("/api/v1/users/login")
        .header(ContentType::JSON)
        .body(json_string!({"user": {"email": EMAIL, "password": PASSWORD}}))
        .dispatch();

    if response.status() == Status::UnprocessableEntity {
        return None;
    }

    let value = response_json_value(response);
    let token = value
        .get("user")
        .and_then(|user| user.get("token"))
        .and_then(|token| token.as_str())
        .map(String::from)
        .expect("Cannot extract token");
    Some(token)
}

/// Register user for use in other tests
pub fn register(client: &Client, first_name: &str, last_name: &str, email: &str, password: &str) {
    let response = client
        .post("/api/v1/users/register")
        .header(ContentType::JSON)
        .body(json_string!({"user": {"firstName": first_name, "lastName": last_name, "email": email, "password": password}}))
        .dispatch();

    match response.status() {
        Status::Ok | Status::Conflict => {} // ok,
        status => panic!("Registration failed: {}", status),
    }
}
