use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiErrorResponse {
    pub message: String,
    pub code: String,
    pub __type: String,
}

impl ApiErrorResponse {
    pub fn new(message: impl Into<String>, code: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: code.into(),
            __type: "AppError".to_string(),
        }
    }
}
