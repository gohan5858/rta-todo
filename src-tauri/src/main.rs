// // Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands;
mod save_data;

use anyhow_tauri::TAResult;
use commands::{save_data::*, timer::*};
use specta::{function::FunctionResult, Type};
use specta_typescript::Typescript;
use tauri_specta::{collect_commands, Builder};

// NOTE: TResultがFunctionResultを実装しておらず、spectaが型を生成できていなかったため以下を追加
pub enum SpectaFunctionResultMarker {}
impl<T: Type> FunctionResult<SpectaFunctionResultMarker> for TAResult<T> {
    fn to_datatype(type_map: &mut specta::TypeMap) -> specta::datatype::FunctionResultVariant {
        specta::datatype::FunctionResultVariant::Value(T::reference(type_map, &[]).inner)
    }
}

// this example exports your types on startup when in debug mode or in a unit test. You can do whatever.
fn main() {
    let builder = Builder::<tauri::Wry>::new()
        // Then register them (separated by a comma)
        .commands(collect_commands![
            initiate_timer,
            pause_timer,
            resume_timer,
            get_current_time,
            load_data,
            set_title,
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
            get_current_elapsed_time,
            reset_current_elapsed_time,
        ]);
    #[cfg(debug_assertions)]
    builder
        .export(Typescript::default(), "../src/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .invoke_handler(builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
