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
            load_data,
            set_theme,
            set_is_auto_start,
            set_is_notification_of_deadline,
            set_is_notification_exceeded_goal_lap_time,
            add_project,
            fetch_project,
            add_todo,
            remove_todo,
            go_to_next_todo,
            update_current_elapsed_time,
            reset_current_elapsed_time,
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
            load_data,
            set_theme,
            set_is_auto_start,
            set_is_notification_of_deadline,
            set_is_notification_exceeded_goal_lap_time,
            add_project,
            fetch_project,
            add_todo,
            remove_todo,
            go_to_next_todo,
            update_current_elapsed_time,
            reset_current_elapsed_time,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
