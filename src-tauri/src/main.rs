// // Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands;
mod save_data;

use commands::{save_data::*, timer::*};
use specta::collect_types;
use tauri_specta::ts;

// this example exports your types on startup when in debug mode or in a unit test. You can do whatever.
fn main() {
    #[cfg(debug_assertions)]
    ts::export(
        collect_types![
            start_timer,
            pause_timer,
            resume_timer,
            reset_timer,
            get_current_time,
            load_data
        ],
        "../src/bindings.ts",
    )
    .unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            start_timer,
            pause_timer,
            resume_timer,
            reset_timer,
            get_current_time,
            load_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
