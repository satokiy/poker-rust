use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("not found")]
    NotFound,

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("internal server error: {0}")]
    Internal(String),

    #[error(transparent)]
    DB(#[from] sea_orm::DbErr),
}
