use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BookInfo {
    pub isbn_13: String,
    pub title: String,
    pub authors: Vec<String>,
    pub publisher: String,
    pub published_date: String,
    pub description: String,
    pub image_url: String,
}
