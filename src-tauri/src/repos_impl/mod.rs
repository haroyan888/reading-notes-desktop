pub mod book;
pub mod memo;

use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::fs;

use crate::repos::RepositoryError;

pub fn read<T: Debug + DeserializeOwned>(path: &str) -> Result<Vec<T>, RepositoryError> {
    let file = fs::read(path).map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
    let data = serde_json::from_slice::<Vec<T>>(&file)
        .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
    Ok(data)
}

pub fn write<T: Debug + DeserializeOwned>(
    path: &str,
    data: Vec<T>,
) -> Result<Vec<T>, RepositoryError> {
    fs::write(path, format!("{:?}", data))
        .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
    read::<T>(path)
}
