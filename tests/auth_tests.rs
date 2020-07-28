//! Test registration and login

mod common;

use common::{EMAIL, FIRST_NAME, LAST_NAME, PASSWORD};

use rocket::http::{ContentType, Status};
use rocket::local::LocalResponse;

#[test]
/// Try registering a new user
fn test_register() {
    let client = common::test_client();
    let response = &mut client
        .post("/api/v1/users/register")
        .header(ContentType::JSON)
        .body(json_string!({"user": {"firstName": FIRST_NAME, "lastName": LAST_NAME, "email": EMAIL, "password": PASSWORD}}))
        .dispatch();

    let status = response.status();
    // If user was already created we should get an Conflict or Ok otherwise.
    //
    // As tests are ran in an independent order `login()` probably has already created smoketest user.
    // And so we gracefully handle "user already exists" error here.
    match status {
        Status::Ok => check_auth_response(response),
        Status::Conflict => check_user_validation_errors(response),
        _ => panic!("Got status: {}", status),
    }
}

#[test]
/// Registration with the same email must fail
fn test_register_with_duplicated_email() {
    let client = common::test_client();
    common::register(
        client,
        FIRST_NAME,
        LAST_NAME,
        "original@example.com",
        PASSWORD,
    );

    let response = &mut client
        .post("/api/v1/users/register")
        .header(ContentType::JSON)
        .body(json_string!({
            "user": {
                "firstName": "new_clone_name",
                "lastName": "new_clone_last_name",
                "email": "original@example.com",
                "password": PASSWORD,
            },
        }))
        .dispatch();

    assert_eq!(Status::Conflict, response.status());

    let value = common::response_json_value(response);
    let success = value.get("success").and_then(|success| success.as_bool());
    let message = value.get("message").and_then(|message| message.as_str());

    assert_eq!(Some(false), success);
    assert_eq!(Some("email is already in use"), message)
}

#[test]
/// Registration with an invalid email format must fail
fn test_register_with_invalid_email_format() {
    let client = common::test_client();
    let response = &mut client
        .post("/api/v1/users/register")
        .header(ContentType::JSON)
        .body(json_string!({"user": {"firstName": FIRST_NAME, "lastName": LAST_NAME, "email": "smoketest", "password": PASSWORD}}))
        .dispatch();

    assert_eq!(Status::UnprocessableEntity, response.status());

    let value = common::response_json_value(response);
    let success = value.get("success").and_then(|success| success.as_bool());
    let message = value.get("message").and_then(|message| message.as_str());

    assert_eq!(Some(false), success);
    assert_eq!(Some("email - email"), message);
}

#[test]
/// Registration with an invalid password format must fail
fn test_register_with_invalid_password_format() {
    let client = common::test_client();
    let response = &mut client
        .post("/api/v1/users/register")
        .header(ContentType::JSON)
        .body(json_string!({"user": {"firstName": FIRST_NAME, "lastName": LAST_NAME, "email": EMAIL, "password": "pw"}}))
        .dispatch();

    assert_eq!(Status::UnprocessableEntity, response.status());

    let value = common::response_json_value(response);
    let success = value.get("success").and_then(|success| success.as_bool());
    let message = value.get("message").and_then(|message| message.as_str());

    assert_eq!(Some(false), success);
    assert_eq!(Some("password - length"), message);
}

#[test]
/// Try logging in, and assure response token is valid
fn test_login() {
    let client = common::test_client();
    let response = &mut client
        .post("/api/v1/users/login")
        .header(ContentType::JSON)
        .body(json_string!({"user": {"email": EMAIL, "password": PASSWORD}}))
        .dispatch();

    check_auth_response(response)
}

#[test]
/// Login with wrong password must fail.
fn test_incorrect_login() {
    let client = common::test_client();
    let response = &mut client
        .post("/api/v1/users/login")
        .header(ContentType::JSON)
        .body(json_string!({"user": {"email": EMAIL, "password": "foobarfoobar"}}))
        .dispatch();

    assert_eq!(Status::Unauthorized, response.status());

    let value = common::response_json_value(response);
    let success = value.get("success").and_then(|success| success.as_bool());
    let message = value.get("message").and_then(|message| message.as_str());

    assert_eq!(Some(false), success);
    assert_eq!(Some("email or password is invalid"), message);
}

#[test]
/// Login with an invalid email format must fail
fn test_login_with_invalid_email_format() {
    let client = common::test_client();
    let response = &mut client
        .post("/api/v1/users/login")
        .header(ContentType::JSON)
        .body(json_string!({"user": {"email": "smoketest", "password": PASSWORD}}))
        .dispatch();

    assert_eq!(Status::UnprocessableEntity, response.status());

    let value = common::response_json_value(response);
    let success = value.get("success").and_then(|success| success.as_bool());
    let message = value.get("message").and_then(|message| message.as_str());

    assert_eq!(Some(false), success);
    assert_eq!(Some("email - email"), message);
}

// Utility

/// Assert that body contains "user" response with expected fields.
fn check_auth_response(response: &mut LocalResponse) {
    let value = common::response_json_value(response);
    let user = value.get("user").expect("must have a 'user' field");

    assert_eq!(EMAIL, user.get("email").expect("must have a 'email' field"));
    assert_eq!(
        FIRST_NAME,
        user.get("firstName")
            .expect("must have a 'firstName' field"),
    );
    assert_eq!(
        LAST_NAME,
        user.get("lastName").expect("must have a 'lastName' field")
    );
    assert!(user.get("token").is_some());
}

/// Catches the registration test, if the email has already been used in the database
fn check_user_validation_errors(response: &mut LocalResponse) {
    let value = common::response_json_value(response);
    let success = value.get("success").and_then(|success| success.as_bool());
    let message = value.get("message").and_then(|message| message.as_str());

    assert_eq!(Some(false), success);
    assert_eq!(Some("email is already in use"), message);
}
