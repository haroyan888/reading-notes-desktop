mod entitiy;
mod repos;
mod repos_impl;

use std::sync::{Arc, Mutex};

type DiGreeter = Arc<Mutex<Box<dyn Greeter>>>;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(greeter: tauri::State<DiGreeter>, name: &str) -> String {
    let greeter = greeter.lock().unwrap();
    greeter.greet(name)
}

trait Greeter: Send + Sync {
    fn greet(&self, name: &str) -> String;
}
#[warn(dead_code)]
struct EnglishGreeter;

impl Greeter for EnglishGreeter {
    fn greet(&self, name: &str) -> String {
        format!("Hello, {}! You've been greeted from Rust!", name)
    }
}

struct SpanishGreeter;

impl Greeter for SpanishGreeter {
    fn greet(&self, name: &str) -> String {
        format!("Hola, {}! Has sido saludado desde Rust!", name)
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let greeter: DiGreeter = Arc::new(Mutex::new(Box::new(SpanishGreeter)));

    tauri::Builder::default()
        .manage(greeter)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
