use std::sync::{LazyLock, Mutex};

use anyhow_tauri::TAResult;

static IS_CHECKED_UPDATE: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

#[tauri::command]
#[specta::specta]
pub fn get_is_checked_update() -> TAResult<bool> {
    if let Ok(checked) = IS_CHECKED_UPDATE.lock() {
        Ok(*checked)
    } else {
        Ok(false)
    }
}

#[tauri::command]
#[specta::specta]
pub fn set_is_checked_update(is_update_checked: bool) -> TAResult<()> {
    if let Ok(mut checked) = IS_CHECKED_UPDATE.lock() {
        *checked = is_update_checked;
    }
    Ok(())
}
