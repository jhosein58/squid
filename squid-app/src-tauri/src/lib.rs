// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::{
    cell::RefCell,
    sync::{mpsc, Arc, OnceLock},
};

use squid_core::FixedSpscQueue;
use tauri::{AppHandle, Emitter};
pub struct AppState {
    pub freq: Vec<f32>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub fn send_frequency(app: tauri::AppHandle, freq: Vec<f32>) {
    app.emit("oscilloscope_waveform", freq).unwrap();
}

static GLOBAL_HANDLE: OnceLock<AppHandle> = OnceLock::new();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(state: Arc<FixedSpscQueue<f32, 512>>) {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![greet])
        .setup(move |app| {
            GLOBAL_HANDLE
                .set(app.handle().clone())
                .expect("AppHandle already initialized");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
pub fn app_handle() -> Option<&'static AppHandle> {
    GLOBAL_HANDLE.get()
}
