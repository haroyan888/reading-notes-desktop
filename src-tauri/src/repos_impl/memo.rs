use async_trait::async_trait;
use std::fs;
use std::path::Path;

use crate::entitiy::memo::Memo;
use crate::repos::memo::{MemoRepository, RepositoryError};
use crate::repos_impl::{read, write};

pub struct MemoRepositoryForJson {
    path: String,
}

impl MemoRepositoryForJson {
    pub fn new(path: &str) -> Result<Self, RepositoryError> {
        if !Path::new(path).is_file() {
            fs::File::create(path).map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
        }
        Ok(MemoRepositoryForJson {
            path: path.to_string(),
        })
    }
}

#[async_trait]
impl MemoRepository for MemoRepositoryForJson {
    async fn find(&self, id: &str) -> Result<Memo, RepositoryError> {
        let data =
            read::<Memo>(&self.path).map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
        let option_memo = data
            .iter()
            .find(|&memo| memo.id == id)
            .ok_or(RepositoryError::NotFound(format!("id:{} is not found", id)))?;
        Ok(option_memo.clone())
    }

    async fn all(&self, isbn_13: &str) -> Result<Vec<Memo>, RepositoryError> {
        let data =
            read::<Memo>(&self.path).map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
        Ok(data
            .iter()
            .filter(|&memo| memo.isbn_13 == isbn_13)
            .cloned()
            .collect::<Vec<Memo>>())
    }

    async fn create(&self, text: &str, isbn_13: &str) -> Result<Memo, RepositoryError> {
        let mut data =
            read::<Memo>(&self.path).map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
        let memo = Memo::new(isbn_13, text);
        data.push(memo.clone());
        write::<Memo>(&self.path, data).map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
        Ok(memo)
    }

    async fn delete(&self, id: &str) -> Result<(), RepositoryError> {
        let data =
            read::<Memo>(&self.path).map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
        let deleted_data = data
            .iter()
            .filter(|&memo| memo.id != id)
            .cloned()
            .collect::<Vec<Memo>>();
        write::<Memo>(&self.path, deleted_data)
            .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
        Ok(())
    }
}
