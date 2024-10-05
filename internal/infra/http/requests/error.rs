use std::collections::HashMap;

use actix_web::http::StatusCode;
use actix_web::{ HttpResponse, ResponseError };
use config::log::debug;
use thiserror::Error;
use validator::{ ValidationError, ValidationErrors, ValidationErrorsKind };

use crate::infra::http::resources::ErrorResponse;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Validation error: {0}")] Validate(#[from] validator::ValidationErrors),
    #[error(transparent)] Deserialize(#[from] DeserializeErrors),
    #[error("Payload error: {0}")] JsonPayloadError(#[from] actix_web::error::JsonPayloadError),
    #[error("Url encoded error: {0}")] UrlEncodedError(#[from] actix_web::error::UrlencodedError),
    #[error("Query error: {0}")] QsError(#[from] serde_qs::Error),
}

#[derive(Error, Debug)]
pub enum DeserializeErrors {
    #[error("Query deserialize error: {0}")] DeserializeQuery(serde_urlencoded::de::Error),
    #[error("Json deserialize error: {0}")] DeserializeJson(serde_json::error::Error),
    #[error("Path deserialize error: {0}")] DeserializePath(serde::de::value::Error),
}

impl From<serde_json::error::Error> for Error {
    fn from(error: serde_json::error::Error) -> Self {
        Error::Deserialize(DeserializeErrors::DeserializeJson(error))
    }
}

impl From<serde_urlencoded::de::Error> for Error {
    fn from(error: serde_urlencoded::de::Error) -> Self {
        Error::Deserialize(DeserializeErrors::DeserializeQuery(error))
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let mut response = ErrorResponse::new(None, None);
        match self {
            Self::Validate(e) => {
                response.field_errors = Some(flatten_errors(e));
            }
            _ => {
                response.error = Some(format!("{}", *self));
            }
        }
        return HttpResponse::build(StatusCode::BAD_REQUEST).json(response);
    }
}

#[inline]
fn flatten_errors(errors: &ValidationErrors) -> HashMap<String, Vec<String>> {
    let mut mapped_errors: HashMap<String, Vec<String>> = HashMap::new();
    for error in errors.errors() {
        match error.1 {
            ValidationErrorsKind::Field(field_errors) => {
                mapped_errors.insert(
                    error.0.to_string(),
                    field_errors
                        .iter()
                        .map(|val_error| val_error.message.as_deref().unwrap().to_string())
                        .collect()
                );
            }
            ValidationErrorsKind::List(list_error) => {
                debug!("{:?}", list_error);
            }
            ValidationErrorsKind::Struct(struct_errors) => {
                debug!("{:?}", struct_errors);
            }
        }
    }
    return mapped_errors;
}

#[inline]
fn _flatten_errors(
    errors: &ValidationErrors,
    path: Option<String>,
    indent: Option<u16>
) -> Vec<(u16, String, &ValidationError)> {
    errors
        .errors()
        .iter()
        .flat_map(|(&field, err)| {
            let indent = indent.unwrap_or(0);
            let actual_path = path
                .as_ref()
                .map(|path| [path.as_str(), field].join("."))
                .unwrap_or_else(|| field.to_owned());
            match err {
                ValidationErrorsKind::Field(field_errors) =>
                    field_errors
                        .iter()
                        .map(|error| (indent, actual_path.clone(), error))
                        .collect::<Vec<_>>(),
                ValidationErrorsKind::List(list_error) =>
                    list_error
                        .iter()
                        .flat_map(|(index, errors)| {
                            let actual_path = format!("{}[{}]", actual_path.as_str(), index);
                            _flatten_errors(errors, Some(actual_path), Some(indent + 1))
                        })
                        .collect::<Vec<_>>(),
                ValidationErrorsKind::Struct(struct_errors) => {
                    _flatten_errors(struct_errors, Some(actual_path), Some(indent + 1))
                }
            }
        })
        .collect::<Vec<_>>()
}
