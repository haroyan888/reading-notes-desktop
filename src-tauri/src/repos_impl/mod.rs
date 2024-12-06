pub mod book;
pub mod reading_note;

use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::fs;

pub fn read<T: Debug + DeserializeOwned>(path: &str) -> Result<Vec<T>, String> {
    let file = fs::read(path).map_err(|e| e.to_string())?;
    let data = serde_json::from_slice::<Vec<T>>(&file).map_err(|e| e.to_string())?;
    Ok(data)
}

pub fn write<T: Debug + DeserializeOwned>(path: &str, data: Vec<T>) -> Result<Vec<T>, String> {
    fs::write(path, format!("{:?}", data)).map_err(|e| e.to_string())?;
    read::<T>(path)
}
