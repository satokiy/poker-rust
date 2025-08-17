use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum AppError {
    #[error("not found: ")]
    NotFound(Option<String>),

    #[error("bad request: ")]
    BadRequest(String),

    #[error("internal error: {0}")]
    Internal(String),
}

impl AppError {
    pub fn not_found() -> Self {
        AppError::NotFound(None)
    }
    pub fn not_found_with_msg(msg: impl Into<String>) -> Self {
        AppError::NotFound(Some(msg.into()))
    }

    pub fn bad_request(msg: impl Into<String>) -> Self {
        AppError::BadRequest(msg.into())
    }
}
