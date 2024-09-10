use crate::save_data::SaveData;
use anyhow_tauri::TAResult;
use std::path::Path;
use tauri::api::path::app_data_dir;

#[tauri::command]
#[specta::specta]
pub fn load_data(app: tauri::AppHandle) -> TAResult<SaveData> {
    let path = app_data_dir(&app.config())
        .and_then(|p| p.into_os_string().into_string().ok())
        .ok_or(anyhow::anyhow!("Failed to get path"))?;
    let save_data = SaveData::load(Path::new(&path))?;

    Ok(save_data)
}
