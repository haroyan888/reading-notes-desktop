use async_trait::async_trait;
use thiserror::Error;

use crate::entitiy::memo::Memo;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Unexpected Error: [{0}]")]
    Unexpected(String),
    #[error("NotFound, id is {0}")]
    NotFound(String),
    #[error("Registered, is is {0}")]
    Registered(String),
}

#[async_trait]
pub trait MemoRepository: Send + Sync + 'static {
    async fn find(&self, id: &str) -> Result<Memo, RepositoryError>;
    async fn all(&self, isbn_13: &str) -> Result<Vec<Memo>, RepositoryError>;
    async fn create(&self, text: &str, isbn_13: &str) -> Result<Memo, RepositoryError>;
    async fn delete(&self, id: &str) -> Result<(), RepositoryError>;
}
