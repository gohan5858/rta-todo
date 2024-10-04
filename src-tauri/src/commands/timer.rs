use anyhow_tauri::TAResult;
use std::{
    sync::{LazyLock, Mutex},
    time::Duration,
};
use tauri::async_runtime::JoinHandle;
use tokio::time;

static IS_PAUSED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
pub static CURRENT_TIME: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));
static CURRENT_TIMER: LazyLock<Mutex<Option<JoinHandle<TAResult<()>>>>> =
    LazyLock::new(|| Mutex::new(None));

#[tauri::command]
#[specta::specta]
pub fn start_timer() -> TAResult<()> {
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
