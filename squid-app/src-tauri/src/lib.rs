// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::sync::{Arc, Mutex};

use tauri::State;
pub struct AppState {
    pub freq: Vec<f32>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_frequency(state: State<Arc<Mutex<AppState>>>) -> Vec<f32> {
    let guard = state.lock().unwrap();
    guard.freq.clone()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(state: Arc<Mutex<AppState>>) {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![greet, get_frequency])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
