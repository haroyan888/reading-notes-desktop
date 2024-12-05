use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;

use crate::entitiy::book::BookInfo;
use crate::repos::book::{BookRepository, BookSearcher};

async fn get_state<T: ?Sized + Send + Sync + 'static>(
    app_handle: &tauri::AppHandle,
) -> Result<tauri::State<Arc<Mutex<T>>>, String> {
    app_handle
        .try_state::<Arc<Mutex<T>>>()
        .ok_or("データ取得に失敗しました".to_string())
}

#[tauri::command]
pub async fn all_book(app_handle: tauri::AppHandle) -> Result<Vec<BookInfo>, String> {
    let state = get_state::<dyn BookRepository>(&app_handle).await?;
    let book_repos = state.lock().await;
    book_repos.all().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn find_book(app_handle: tauri::AppHandle, isbn_13: String) -> Result<BookInfo, String> {
    let state = get_state::<dyn BookRepository>(&app_handle).await?;
    let book_repos = state.lock().await;
    book_repos.find(&isbn_13).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_book(
    app_handle: tauri::AppHandle,
    isbn_13: String,
) -> Result<BookInfo, String> {
    let state_repos = get_state::<dyn BookRepository>(&app_handle).await?;
    let book_repos = state_repos.lock().await;
    let state_searcher = get_state::<dyn BookSearcher>(&app_handle).await?;
    let book_searcher = state_searcher.lock().await;

    let book = book_searcher
        .find(&isbn_13)
        .await
        .map_err(|e| e.to_string())?;

    book_repos.create(book).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_book(app_handle: tauri::AppHandle, isbn_13: String) -> Result<(), String> {
    let state = get_state::<dyn BookRepository>(&app_handle).await?;
    let book_repos = state.lock().await;

    book_repos.delete(&isbn_13).await.map_err(|e| e.to_string())
}
