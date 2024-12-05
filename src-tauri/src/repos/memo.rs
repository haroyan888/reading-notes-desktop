use async_trait::async_trait;
use serde::Serialize;
use std::sync::{Arc, Mutex};

use super::RepositoryError;

pub type DiMemoRepository = Arc<Mutex<Box<dyn MemoRepository>>>;

#[derive(Serialize, Debug, PartialEq)]
pub struct Memo {
    pub id: String,
    pub isbn_13: String,
    pub text: String,
}

#[async_trait]
pub trait MemoRepository: Clone + Send + Sync + 'static {
    async fn find(&self, id: &str) -> Result<Memo, RepositoryError>;
    async fn find_all(&self, isbn_13: &str) -> Result<Vec<Memo>, RepositoryError>;
    async fn create(&self, text: String, isbn_13: &str) -> Result<Memo, RepositoryError>;
    async fn delete(&self, id: &str) -> Result<(), RepositoryError>;
}
