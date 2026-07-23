use std::{collections::HashMap, fmt::Debug};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiErrorResponse {
    pub message: String,
    pub code: String,
    pub data: HashMap<String, String>,
    pub __type: String,
}

impl ApiErrorResponse {
    pub fn new(message: impl Into<String>, code: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: code.into(),
            data: HashMap::new(),
            __type: "ApiErrorResponse".to_string(),
        }
    }

    pub fn new_with_data(
        message: impl Into<String>,
        code: impl Into<String>,
        data: HashMap<String, String>,
    ) -> Self {
        Self {
            message: message.into(),
            code: code.into(),
            data,
            __type: "ApiErrorResponse".to_string(),
        }
    }
}

impl From<validator::ValidationErrors> for ApiErrorResponse {
    fn from(errors: validator::ValidationErrors) -> Self {
        let data = errors
            .field_errors()
            .iter()
            .filter_map(|(field, errors)| {
                errors.first().map(|err| {
                    (
                        field.to_string(),
                        err.message
                            .as_ref()
                            .map(|m| m.to_string())
                            .unwrap_or_else(|| "Invalid value".to_string()),
                    )
                })
            })
            .collect();

        Self::new_with_data("", "VALIDATION_FAILED", data)
    }
}
