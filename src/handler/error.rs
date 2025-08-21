use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

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

impl AppError {
    pub fn to_response(&self) -> Response {
        let status = match self {
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, Json(ErrorResponse::from_err(self.clone()))).into_response()
    }
}
