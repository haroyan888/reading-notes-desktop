use std::fs;
use std::path::Path;

use async_trait::async_trait;

use super::super::entitiy::book::BookInfo;
use super::super::repos::{
    book::{BookRepository, BookSearcher, SearchBooksResult},
    RepositoryError,
};

#[derive(Clone)]
pub struct BookRepositoryForJson {
    path: String,
}

impl BookRepositoryForJson {
    pub fn new(path: &str) -> Self {
        if !Path::new(path).is_file() {
            fs::File::create(path).expect("ファイルの作成に失敗しました");
        }
        BookRepositoryForJson {
            path: path.to_string(),
        }
    }
}

#[async_trait]
impl BookRepository for BookRepositoryForJson {
    async fn find(&self, isbn_13: &str) -> Result<BookInfo, RepositoryError> {
        let file = fs::read(&self.path).map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
        let data = serde_json::from_slice::<Vec<BookInfo>>(&file)
            .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
        let option_book = data
            .iter()
            .find(|&book| book.isbn_13 == isbn_13)
            .ok_or_else(|| RepositoryError::NotFound(format!("isbn:{} is not found", isbn_13)))?;
        Ok(option_book.clone())
    }
    async fn find_all(&self) -> Result<Vec<BookInfo>, RepositoryError>;
    async fn create(&self, payload: BookInfo) -> Result<BookInfo, RepositoryError>;
    async fn delete(&self, isbn_13: &str) -> Result<(), RepositoryError>;
}

#[derive(Clone)]
pub struct BookSearcherForGoogleAPI {}

#[async_trait]
impl BookSearcher for BookSearcherForGoogleAPI {
    async fn find(&self, isbn_13: &str) -> Result<BookInfo, RepositoryError> {
        const URL: &str = "https://www.googleapis.com/books/v1/volumes?q=isbn:";
        let res = reqwest::get(format!("{}{}", URL, &isbn_13))
            .await
            .map_err(|e| RepositoryError::Unexpected(e.to_string()))?
            .text()
            .await
            .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;

        let search_books_result = serde_json::from_str::<SearchBooksResult>(&res)
            .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;

        if search_books_result.items.is_empty() {
            return Err(RepositoryError::NotFound(format!(
                "isbn13:{} is not found.",
                isbn_13
            )));
        }

        // isbnで一意に検索しているためインデックス0で指定
        let book = search_books_result.items[0].volume_info.to_book_info();

        // Googleがisbn不一致でも良しなに変換してくれるが、ここでははじく
        if isbn_13 != book.isbn_13 {
            return Err(RepositoryError::NotFound(format!(
                "isbn13:{} is not found.",
                isbn_13
            )));
        }

        Ok(book)
    }
}
