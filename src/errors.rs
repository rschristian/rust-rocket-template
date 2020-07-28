use rocket::http::Status;
use rocket::request::Request;
use rocket::response::status;
use rocket::response::{self, Responder};
use rocket_contrib::json::Json;
use validator::{Validate, ValidationError, ValidationErrors, ValidationErrorsKind::Field};

#[derive(Debug)]
pub struct Errors {
    status: Status,
    message: String,
}

impl Errors {
    pub fn new(status: Status, message: String) -> Self {
        Self { status, message }
    }
}

impl<'r> Responder<'r> for Errors {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        status::Custom(
            self.status,
            Json(json!({ "success": false, "message": self.message })),
        )
        .respond_to(req)
    }
}

pub struct FieldValidator {
    errors: ValidationErrors,
}

impl Default for FieldValidator {
    fn default() -> Self {
        Self {
            errors: ValidationErrors::new(),
        }
    }
}

impl FieldValidator {
    pub fn validate<T: Validate>(model: &T) -> Self {
        Self {
            errors: model.validate().err().unwrap_or_else(ValidationErrors::new),
        }
    }

    /// Convenience method to trigger early returns with ? operator.
    pub fn check(self) -> Result<(), Errors> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            let mut error_message = "".to_owned();
            for (field, field_errors) in self.errors.into_errors() {
                if let Field(field_errors) = field_errors {
                    error_message = format!("{} - {}", field, field_errors[0].code);
                }
            }

            Err(Errors {
                status: Status::UnprocessableEntity,
                message: error_message,
            })
        }
    }

    pub fn extract<T>(&mut self, field_name: &'static str, field: Option<T>) -> T
    where
        T: Default,
    {
        field.unwrap_or_else(|| {
            self.errors
                .add(field_name, ValidationError::new("can't be blank"));
            T::default()
        })
    }
}
