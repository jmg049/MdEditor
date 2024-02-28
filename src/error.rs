use std::fmt::Debug;
use thiserror::Error;

pub(crate) type MdResult<T> = Result<T, MdError>;

#[derive(Error, Debug)]
pub(crate) enum MdError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid action: {0}")]
    InvalidAction(String),
}
