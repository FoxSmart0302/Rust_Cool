use thiserror::Error;

pub type XResult<T> = Result<T, XError>;

#[derive(Error, Debug)]
pub enum XError {
    #[error("sqlx err: {0}")]
    Sqlx(#[from] sqlx::Error),
}
