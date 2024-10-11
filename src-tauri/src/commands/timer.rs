use anyhow_tauri::TAResult;
use std::{
    path::Path,
    sync::{LazyLock, Mutex},
    time::Duration,
};
use tauri::{api::path::app_data_dir, async_runtime::JoinHandle};
use tokio::time;

use crate::save_data::SaveData;

pub static IS_PAUSED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
pub static CURRENT_TIME: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));
static CURRENT_TIMER: LazyLock<Mutex<Option<JoinHandle<TAResult<()>>>>> =
    LazyLock::new(|| Mutex::new(None));

#[tauri::command]
#[specta::specta]
pub fn initiate_timer(app: tauri::AppHandle, project_id: uuid::Uuid) -> TAResult<u32> {
    let Ok(mut current_timer) = CURRENT_TIMER.lock() else {
        return Err(anyhow::anyhow!("Failed to acquire current_time lock").into());
    };
    let Ok(mut is_paused) = IS_PAUSED.lock() else {
        return Err(anyhow::anyhow!("Failed to acquire is_paused lock").into());
    };
    let Ok(mut current_time) = CURRENT_TIME.lock() else {
        return Err(anyhow::anyhow!("Failed to acquire current_time lock").into());
    };

    let path = app_data_dir(&app.config())
        .and_then(|p| p.into_os_string().into_string().ok())
        .ok_or(anyhow::anyhow!("Failed to get path"))?;
    let save_data = SaveData::load(Path::new(&path))?;
    let previous_time = save_data
        .projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or(anyhow::anyhow!("Failed to find project"))?;
    *is_paused = true;
    *current_time = previous_time.current_elapsed_time as u32;
    // タイマーを一つに限定
    if current_timer.is_some() {
        return Ok(*current_time);
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
    Ok(*current_time)
}
#[tauri::command]
#[specta::specta]
pub fn pause_timer() -> TAResult<()> {
    let Ok(mut is_paused) = IS_PAUSED.lock() else {
        return Err(anyhow::anyhow!("Failed to acquire lock").into());
    };
    *is_paused = true;
    Ok(())
}
#[tauri::command]
#[specta::specta]
pub fn resume_timer() -> TAResult<()> {
    let Ok(mut is_paused) = IS_PAUSED.lock() else {
        return Err(anyhow::anyhow!("Failed to acquire lock").into());
    };
    *is_paused = false;
    Ok(())
}
#[tauri::command]
#[specta::specta]
pub fn reset_timer() -> TAResult<()> {
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
pub fn get_current_time() -> TAResult<u32> {
    let Ok(current_time) = CURRENT_TIME.lock() else {
        return Err(anyhow::anyhow!("Failed to acquire lock").into());
    };
    Ok(*current_time)
}
#[tauri::command]
#[specta::specta]
pub fn reset_current_elapsed_time() -> TAResult<()> {
    IS_PAUSED
        .lock()
        .map(|mut is_paused| {
            *is_paused = true;
        })
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
    CURRENT_TIME
        .lock()
        .map(|mut current_time| {
            *current_time = 0;
        })
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
    Ok(())
}
