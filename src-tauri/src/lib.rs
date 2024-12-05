mod entitiy;
mod handler;
mod repos;
mod repos_impl;

use std::{
    fs,
    sync::{Arc, Mutex},
};
use tauri::Manager;

use handler::book::{all_book, create_book, delete_book, find_book};
use repos_impl::book::BookRepositoryForJson;

fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let path = app.path().app_data_dir()?.join("book-repos.json");
    if !path.is_file() {
        fs::File::create(path.clone())?;
    }
    app.manage(Arc::new(Mutex::new(Box::new(BookRepositoryForJson::new(
        path.to_str().unwrap(),
    )))));
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(setup)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            all_book,
            find_book,
            create_book,
            delete_book
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
