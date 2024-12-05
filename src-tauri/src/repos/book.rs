use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::super::entitiy::book::BookInfo;
use super::RepositoryError;

#[async_trait]
pub trait BookRepository: Send + Sync + 'static {
    async fn find(&self, isbn_13: &str) -> Result<BookInfo, RepositoryError>;
    async fn all(&self) -> Result<Vec<BookInfo>, RepositoryError>;
    async fn create(&self, payload: BookInfo) -> Result<BookInfo, RepositoryError>;
    async fn delete(&self, isbn_13: &str) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait BookSearcher: Send + Sync + 'static {
    async fn find(&self, isbn_13: &str) -> Result<BookInfo, RepositoryError>;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Identifier {
    #[serde(rename = "type")]
    identifier_type: String,
    identifier: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ImageLinks {
    thumbnail: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VolumeInfoResult {
    title: String,
    description: String,
    authors: Vec<String>,
    publisher: String,
    published_date: String,
    image_links: ImageLinks,
    industry_identifiers: Vec<Identifier>,
}

impl VolumeInfoResult {
    pub fn to_book_info(&self) -> BookInfo {
        BookInfo {
            isbn_13: self
                .industry_identifiers
                .iter()
                .find(|identifier| identifier.identifier_type == "ISBN_13")
                .unwrap()
                .identifier
                .clone(),
            title: self.title.clone(),
            description: self.description.clone(),
            authors: self.authors.clone(),
            publisher: self.publisher.clone(),
            published_date: self.published_date.clone(),
            image_url: self.image_links.thumbnail.clone(),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BookInfoResult {
    pub volume_info: VolumeInfoResult,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchBooksResult {
    pub items: Vec<BookInfoResult>,
}
