// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::{
    cell::RefCell,
    sync::{mpsc, Arc, OnceLock},
};

use squid_core::{Event, EventData, FixedSpscQueue};
use squid_engine::StreamContext;
use tauri::{AppHandle, Emitter, State};
pub struct AppState {
    pub freq: Vec<f32>,
}

#[tauri::command]
fn push_note_event(state: State<Arc<StreamContext>>, note: u8) {
    let _ = state.events.push(Event {
        timing: 0,
        data: EventData::NoteOn {
            note: note,
            velocity: 0,
        },
    });
}

#[tauri::command]
fn note_off_event(state: State<Arc<StreamContext>>, note: u8) {
    let _ = state.events.push(Event {
        timing: 0,
        data: EventData::NoteOff { note: note },
    });
}

#[tauri::command]
fn set_f1(state: State<Arc<StreamContext>>, f1: f32) {
    state
        .f1
        .store((f1 * 100.) as u8, std::sync::atomic::Ordering::Relaxed);
}

#[tauri::command]
fn set_f2(state: State<Arc<StreamContext>>, f2: f32) {
    state
        .f2
        .store((f2 * 100.) as u8, std::sync::atomic::Ordering::Relaxed);
}

#[tauri::command]
fn set_f3(state: State<Arc<StreamContext>>, f3: f32) {
    state
        .f3
        .store((f3 * 100.) as u8, std::sync::atomic::Ordering::Relaxed);
}

pub fn send_frequency(app: tauri::AppHandle, freq: Vec<f32>) {
    app.emit("oscilloscope_waveform", freq).unwrap();
}

static GLOBAL_HANDLE: OnceLock<AppHandle> = OnceLock::new();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(state: Arc<StreamContext>) {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            push_note_event,
            note_off_event,
            set_f1,
            set_f2,
            set_f3
        ])
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
