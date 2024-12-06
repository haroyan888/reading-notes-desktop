mod entitiy;
mod handler;
mod repos;
mod repos_impl;

use std::sync::{Arc, Mutex};
use tauri::Manager;

use handler::book::{all_book, create_book, delete_book, find_book};
use handler::reading_note::{create_reading_note, delete_reading_note, get_reading_notes};
use repos_impl::{book::BookRepositoryForJson, reading_note::ReadingNoteRepositoryForJson};

fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let base_dir = app.path().app_data_dir()?;
    let book_repos_path = base_dir.join("book-repos.json");
    app.manage(Arc::new(Mutex::new(BookRepositoryForJson::new(
        book_repos_path.to_str().unwrap(),
    )?)));
    let reading_note_repos_path = base_dir.join("reading_note-repos.json");
    app.manage(Arc::new(Mutex::new(ReadingNoteRepositoryForJson::new(
        reading_note_repos_path.to_str().unwrap(),
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
            delete_book,
            get_reading_notes,
            create_reading_note,
            delete_reading_note,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
