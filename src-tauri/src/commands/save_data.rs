use crate::save_data::SaveData;
use anyhow_tauri::TAResult;
use std::path::Path;
use tauri::{api::path::app_data_dir, Manager};

#[tauri::command]
#[specta::specta]
pub fn load_data(app: tauri::AppHandle) -> TAResult<SaveData> {
    let path = app_data_dir(&app.config())
        .and_then(|p| p.into_os_string().into_string().ok())
        .ok_or(anyhow::anyhow!("Failed to get path"))?;
    let save_data = SaveData::load(Path::new(&path))?;

    Ok(save_data)
}

#[tauri::command]
#[specta::specta]
pub fn set_theme(app: tauri::AppHandle, theme: String) -> TAResult<()> {
    let path = app_data_dir(&app.config())
        .and_then(|p| p.into_os_string().into_string().ok())
        .ok_or(anyhow::anyhow!("Failed to get path"))?;
    let mut save_data = SaveData::load(Path::new(&path))?;
    save_data.theme = theme;
    SaveData::save(save_data, Path::new(&path))?;
    app.emit_all("update-setting", ()).map_err(|e| anyhow::anyhow!(e))?;
    Ok(())
}
