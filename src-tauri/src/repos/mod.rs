pub mod book;
pub mod memo;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Unexpected Error: [{0}]")]
    Unexpected(String),
    #[error("NotFound, isbn is {0}")]
    NotFound(String),
    #[error("Registered, isbn is {0}")]
    Registered(String),
}
