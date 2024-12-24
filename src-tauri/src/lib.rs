mod commands;
mod events;
mod save_data;

use anyhow_tauri::TAResult;
use commands::{save_data::*, timer::*};
use events::timer::UpdaterIsPaused;
use specta::{function::FunctionResult, Type};
use specta_typescript::Typescript;
use tauri_specta::{collect_commands, collect_events, Builder};

// NOTE: TResultがFunctionResultを実装しておらず、spectaが型を生成できていなかったため以下を追加
pub enum SpectaFunctionResultMarker {}
impl<T: Type> FunctionResult<SpectaFunctionResultMarker> for TAResult<T> {
    fn to_datatype(type_map: &mut specta::TypeMap) -> specta::datatype::FunctionResultVariant {
        specta::datatype::FunctionResultVariant::Value(T::reference(type_map, &[]).inner)
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = Builder::<tauri::Wry>::new()
        .events(collect_events![UpdaterIsPaused])
        .commands(collect_commands![
            get_is_paused,
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
            set_is_complete_project,
            add_project,
            fetch_project,
            remove_project,
            add_todo,
            remove_todo,
            update_todo_list,
            go_to_next_todo,
            update_current_elapsed_time,
            get_current_elapsed_time,
            reset_current_elapsed_time,
        ]);
    #[cfg(debug_assertions)]
    builder
        .export(
            Typescript::default().header("// @ts-nocheck\n"),
            "../src/bindings.ts",
        )
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);

            Ok(())
        })
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
