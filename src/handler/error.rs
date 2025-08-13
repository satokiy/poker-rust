use serde::{Deserialize, Serialize};

use crate::error::error::AppError;

#[derive(Serialize)]
pub struct ErrorResponse {
    message: String,
}

impl ErrorResponse {
    pub fn from_err(err: AppError) -> Self {
        ErrorResponse {
            message: err.to_string(),
        }
    }
}
