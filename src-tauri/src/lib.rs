mod entitiy;
mod handler;
mod repos;
mod repos_impl;

use std::sync::{Arc, Mutex};
use tauri::Manager;

use handler::book::{all_book, create_book, delete_book, find_book};
use repos_impl::{book::BookRepositoryForJson, memo::MemoRepositoryForJson};

fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let base_dir = app.path().app_data_dir()?;
    let book_repos_path = base_dir.join("book-repos.json");
    app.manage(Arc::new(Mutex::new(BookRepositoryForJson::new(
        book_repos_path.to_str().unwrap(),
    )?)));
    let memo_repos_path = base_dir.join("memo-repos.json");
    app.manage(Arc::new(Mutex::new(MemoRepositoryForJson::new(
        memo_repos_path.to_str().unwrap(),
    )?)));
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
