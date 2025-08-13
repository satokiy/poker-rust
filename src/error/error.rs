use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("not found")]
    NotFound(),

    #[error("internal error: {0}")]
    Internal(String),
}
