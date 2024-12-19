mod commands;
mod save_data;

use anyhow_tauri::TAResult;
use commands::{save_data::*, timer::*};
use specta::{function::FunctionResult, Type};
use specta_typescript::Typescript;
use tauri_plugin_updater::UpdaterExt;
use tauri_specta::{collect_commands, Builder};

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
            set_is_complete_project,
            add_project,
            fetch_project,
            remove_project,
            add_todo,
            remove_todo,
            update_todo_item_title,
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
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                update(handle).await.unwrap();
            });
            Ok(())
        })
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
        let mut downloaded = 0;

        // alternatively we could also call update.download() and update.install() separately
        update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                    println!("downloaded {downloaded} from {content_length:?}");
                },
                || {
                    println!("download finished");
                },
            )
            .await?;

        println!("update installed");
        app.restart();
    }

    Ok(())
}
