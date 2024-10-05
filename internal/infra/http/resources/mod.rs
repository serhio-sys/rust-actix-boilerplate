use std::collections::HashMap;

use serde::Serialize;

pub mod user_resouce;

#[derive(Serialize, Clone, PartialEq)]
pub struct BasedListResponse<T> where T: Serialize {
    pub data: Vec<T>,
    pub total: u64,
    pub page: u32,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_errors: Option<HashMap<&'static str, validator::ValidationErrorsKind>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl ErrorResponse {
    pub fn new(
        error: Option<String>,
        field_errors: Option<HashMap<&'static str, validator::ValidationErrorsKind>>
    ) -> Self {
        return ErrorResponse { field_errors, error };
    }
}
