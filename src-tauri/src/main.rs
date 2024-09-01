// // Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod save_data;

use anyhow_tauri::TAResult;
use save_data::SaveData;
use specta::collect_types;
use std::{path::Path, sync::{LazyLock, Mutex}, time::Duration};
use tauri::{api::path::app_data_dir, async_runtime::JoinHandle};
use tauri_specta::ts;
use tokio::time;

static IS_PAUSED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
static CURRENT_TIME: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));
static CURRENT_TIMER: LazyLock<Mutex<Option<JoinHandle<TAResult<()>>>>> =
    LazyLock::new(|| Mutex::new(None));

#[tauri::command]
#[specta::specta]
fn start_timer() -> TAResult<()> {
    let Ok(mut current_timer) = CURRENT_TIMER.lock() else {
        return Err(anyhow::anyhow!("Failed to acquire lock").into());
    };
    // タイマーを一つに限定
    if current_timer.is_some() {
        return Ok(());
    }

    let join = tauri::async_runtime::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(1));

        loop {
            interval.tick().await;

            let Ok(is_paused) = IS_PAUSED.lock() else {
                return Err(anyhow::anyhow!("Failed to acquire lock").into());
            };
            // 一時停止状態でないかを確認
            if *is_paused {
                continue;
            }
            let Ok(mut current_time) = CURRENT_TIME.lock() else {
                return Err(anyhow::anyhow!("Failed to acquire lock").into());
            };
            *current_time += 1;
        }
    });

    *current_timer = Some(join);
    Ok(())
}
#[tauri::command]
#[specta::specta]
fn pause_timer() -> TAResult<()> {
    let Ok(mut is_paused) = IS_PAUSED.lock() else {
        return Err(anyhow::anyhow!("Failed to acquire lock").into());
    };
    *is_paused = true;
    Ok(())
}
#[tauri::command]
#[specta::specta]
fn resume_timer() -> TAResult<()> {
    let Ok(mut is_paused) = IS_PAUSED.lock() else {
        return Err(anyhow::anyhow!("Failed to acquire lock").into());
    };
    *is_paused = false;
    Ok(())
}
#[tauri::command]
#[specta::specta]
fn reset_timer() -> TAResult<()> {
    let Ok(mut current_time) = CURRENT_TIME.lock() else {
        return Err(anyhow::anyhow!("Failed to acquire lock").into());
    };
    *current_time = 0;

    let Ok(mut is_paused) = IS_PAUSED.lock() else {
        return Err(anyhow::anyhow!("Failed to acquire lock").into());
    };
    *is_paused = false;

    if let Some(join) = CURRENT_TIMER.lock().unwrap().take() {
        join.abort();
    }
    Ok(())
}
#[tauri::command]
#[specta::specta]
fn get_current_time() -> TAResult<u32> {
    let Ok(current_time) = CURRENT_TIME.lock() else {
        return Err(anyhow::anyhow!("Failed to acquire lock").into());
    };
    Ok(*current_time)
}

#[tauri::command]
#[specta::specta]
fn load_data(app: tauri::AppHandle) -> TAResult<SaveData> {
    let path = app_data_dir(&app.config())
        .and_then(|p| p.into_os_string().into_string().ok())
        .ok_or(anyhow::anyhow!("Failed to get path"))?;
    let save_data = SaveData::load(Path::new(&path))?;

    Ok(save_data)
}

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
