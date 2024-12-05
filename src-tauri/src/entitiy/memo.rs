use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Memo {
    pub id: String,
    pub isbn_13: String,
    pub text: String,
}

impl Memo {
    pub fn new(isbn_13: &str, text: &str) -> Self {
        Memo {
            id: uuid::Uuid::new_v4().to_string(),
            isbn_13: isbn_13.to_string(),
            text: text.to_string(),
        }
    }
}
