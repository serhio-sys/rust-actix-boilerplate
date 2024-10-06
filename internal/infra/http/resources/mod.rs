use std::collections::HashMap;

use serde::Serialize;

pub mod user_resource;

#[derive(Serialize, Clone, PartialEq)]
pub struct BasedListResponse<T> where T: Serialize {
    pub data: Vec<T>,
    pub total: u64,
    pub page: u32,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_errors: Option<HashMap<String, Vec<String>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl ErrorResponse {
    pub fn new_error(error: Option<String>) -> Self {
        return ErrorResponse { field_errors: None, error };
    }

    pub fn new_field_errors(field_errors: Option<HashMap<String, Vec<String>>>) -> Self {
        return ErrorResponse { field_errors, error: None };
    }
}
