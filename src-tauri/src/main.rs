// // Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use specta::collect_types;
use std::sync::LazyLock;
use std::{sync::Mutex, time::Duration};
use tauri_specta::ts;
use tokio::time;

static IS_PAUSED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
static CURRENT_TIME: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
#[specta::specta]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
#[specta::specta]
fn start_timer() {
    // タイマーを一つに限定
    if *CURRENT_TIME.lock().unwrap() != 0 {
        return;
    }

    tauri::async_runtime::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(1));

        loop {
            interval.tick().await;

            // 一時停止状態でないかを確認
            if *IS_PAUSED.lock().unwrap() {
                continue;
            }
            *CURRENT_TIME.lock().unwrap() += 1;
        }
    });
}
#[tauri::command]
#[specta::specta]
fn pause_timer() {
    *IS_PAUSED.lock().unwrap() = true;
}
#[tauri::command]
#[specta::specta]
fn resume_timer() {
    *IS_PAUSED.lock().unwrap() = false;
}
#[tauri::command]
#[specta::specta]
fn reset_timer() {
    *CURRENT_TIME.lock().unwrap() = 0;
    *IS_PAUSED.lock().unwrap() = false;
}
#[tauri::command]
#[specta::specta]
fn get_current_time() -> Result<u32, String> {
    CURRENT_TIME
        .lock()
        .map(|guard| *guard)
        .map_err(|_| "Failed to acquire lock".to_string())
}

// this example exports your types on startup when in debug mode or in a unit test. You can do whatever.
fn main() {
    #[cfg(debug_assertions)]
    ts::export(
        collect_types![
            greet,
            start_timer,
            pause_timer,
            resume_timer,
            reset_timer,
            get_current_time
        ],
        "../src/bindings.ts",
    )
    .unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            start_timer,
            pause_timer,
            resume_timer,
            reset_timer,
            get_current_time
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
