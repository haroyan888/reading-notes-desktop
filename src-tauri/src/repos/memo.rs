use async_trait::async_trait;

use super::RepositoryError;
use crate::entitiy::memo::Memo;

#[async_trait]
pub trait MemoRepository: Send + Sync + 'static {
    async fn find(&self, id: &str) -> Result<Memo, RepositoryError>;
    async fn all(&self, isbn_13: &str) -> Result<Vec<Memo>, RepositoryError>;
    async fn create(&self, text: &str, isbn_13: &str) -> Result<Memo, RepositoryError>;
    async fn delete(&self, id: &str) -> Result<(), RepositoryError>;
}
