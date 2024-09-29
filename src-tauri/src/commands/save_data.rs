use crate::save_data::{Project, SaveData, Todo};
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
    app.emit_all("update-setting", ())
        .map_err(|e| anyhow::anyhow!(e))?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn set_is_auto_start(app: tauri::AppHandle, is_auto_start: bool) -> TAResult<()> {
    let path = app_data_dir(&app.config())
        .and_then(|p| p.into_os_string().into_string().ok())
        .ok_or(anyhow::anyhow!("Failed to get path"))?;
    let mut save_data = SaveData::load(Path::new(&path))?;
    save_data.is_auto_start = is_auto_start;
    SaveData::save(save_data, Path::new(&path))?;
    app.emit_all("update-setting", ())
        .map_err(|e| anyhow::anyhow!(e))?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn set_is_notification_of_deadline(
    app: tauri::AppHandle,
    is_notification_of_deadline: bool,
) -> TAResult<()> {
    let path = app_data_dir(&app.config())
        .and_then(|p| p.into_os_string().into_string().ok())
        .ok_or(anyhow::anyhow!("Failed to get path"))?;
    let mut save_data = SaveData::load(Path::new(&path))?;
    save_data.is_notification_of_deadline = is_notification_of_deadline;
    SaveData::save(save_data, Path::new(&path))?;
    app.emit_all("update-setting", ())
        .map_err(|e| anyhow::anyhow!(e))?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn set_is_notification_exceeded_goal_lap_time(
    app: tauri::AppHandle,
    is_notification_exceeded_goal_lap_time: bool,
) -> TAResult<()> {
    let path = app_data_dir(&app.config())
        .and_then(|p| p.into_os_string().into_string().ok())
        .ok_or(anyhow::anyhow!("Failed to get path"))?;
    let mut save_data = SaveData::load(Path::new(&path))?;
    save_data.is_notification_exceeded_goal_lap_time = is_notification_exceeded_goal_lap_time;
    SaveData::save(save_data, Path::new(&path))?;
    app.emit_all("update-setting", ())
        .map_err(|e| anyhow::anyhow!(e))?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn add_project(app: tauri::AppHandle, title: String, deadline: Option<String>) -> TAResult<()> {
    let deadline = deadline
        .and_then(|d| chrono::DateTime::parse_from_str(&d, "%Y-%m-%d %H:%M").ok())
        .map(|dt| dt.with_timezone(&chrono::Utc));
    let path = app_data_dir(&app.config())
        .and_then(|p| p.into_os_string().into_string().ok())
        .ok_or(anyhow::anyhow!("Failed to get path"))?;
    let mut save_data = SaveData::load(Path::new(&path))?;
    save_data.projects.push(Project {
        id: uuid::Uuid::now_v7(),
        title,
        deadline,
        completed: false,
        todo_list: Vec::new(),
    });
    SaveData::save(save_data, Path::new(&path))?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn add_todo(
    app: tauri::AppHandle,
    project_id: uuid::Uuid,
    title: String,
) -> TAResult<(Vec<Todo>, Vec<Todo>)> {
    let path = app_data_dir(&app.config())
        .and_then(|p| p.into_os_string().into_string().ok())
        .ok_or(anyhow::anyhow!("Failed to get path"))?;
    let mut save_data = SaveData::load(Path::new(&path))?;
    let project = save_data
        .projects
        .iter_mut()
        .find(|p| p.id == project_id)
        .ok_or(anyhow::anyhow!("Failed to find project"))?;
    let new_todo = Todo {
        id: uuid::Uuid::now_v7(),
        title,
        lap_time: None,
        elapsed_time: None,
        checked: false,
        checkable: project.todo_list.len() == 0 || project.todo_list.iter().all(|t| !t.checkable),
        branch_name: None,
    };
    project.todo_list.push(new_todo);
    let (unchecked_todo_list, checked_todo_list): (Vec<Todo>, Vec<Todo>) = project
        .todo_list
        .clone()
        .into_iter()
        .partition(|t| !t.checked);
    SaveData::save(save_data, Path::new(&path))?;
    Ok((unchecked_todo_list, checked_todo_list))
}
    SaveData::save(save_data, Path::new(&path))?;
    Ok(new_todo)
}
