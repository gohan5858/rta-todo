use crate::save_data::{Project, SaveData, TodoList};
use anyhow_tauri::TAResult;
use std::path::Path;
use tauri::{Emitter, Manager};

#[tauri::command]
#[specta::specta]
pub fn load_data(app: tauri::AppHandle) -> TAResult<SaveData> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?
        .into_os_string()
        .into_string()
        .map_err(|_| anyhow::anyhow!("Failed to get path"))?;
    let save_data = SaveData::load(Path::new(&path))?;

    Ok(save_data)
}

#[tauri::command]
#[specta::specta]
pub fn set_title(app: tauri::AppHandle, project_id: uuid::Uuid, title: String) -> TAResult<()> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?
        .into_os_string()
        .into_string()
        .map_err(|_| anyhow::anyhow!("Failed to get path"))?;
    let mut save_data = SaveData::load(Path::new(&path))?;
    let project = save_data
        .projects
        .iter_mut()
        .find(|p| p.id == project_id)
        .ok_or(anyhow::anyhow!("Failed to find project"))?;
    project.title = title;
    SaveData::save(save_data, Path::new(&path))?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn set_theme(app: tauri::AppHandle, theme: String) -> TAResult<()> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?
        .into_os_string()
        .into_string()
        .map_err(|_| anyhow::anyhow!("Failed to get path"))?;
    let mut save_data = SaveData::load(Path::new(&path))?;
    save_data.theme = theme;
    SaveData::save(save_data, Path::new(&path))?;
    app.emit("update-setting", ())
        .map_err(|e| anyhow::anyhow!(e))?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn set_is_auto_start(app: tauri::AppHandle, is_auto_start: bool) -> TAResult<()> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?
        .into_os_string()
        .into_string()
        .map_err(|_| anyhow::anyhow!("Failed to get path"))?;
    let mut save_data = SaveData::load(Path::new(&path))?;
    save_data.is_auto_start = is_auto_start;
    SaveData::save(save_data, Path::new(&path))?;
    app.emit("update-setting", ())
        .map_err(|e| anyhow::anyhow!(e))?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn set_is_notification_of_deadline(
    app: tauri::AppHandle,
    is_notification_of_deadline: bool,
) -> TAResult<()> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?
        .into_os_string()
        .into_string()
        .map_err(|_| anyhow::anyhow!("Failed to get path"))?;
    let mut save_data = SaveData::load(Path::new(&path))?;
    save_data.is_notification_of_deadline = is_notification_of_deadline;
    SaveData::save(save_data, Path::new(&path))?;
    app.emit("update-setting", ())
        .map_err(|e| anyhow::anyhow!(e))?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn set_is_notification_exceeded_goal_lap_time(
    app: tauri::AppHandle,
    is_notification_exceeded_goal_lap_time: bool,
) -> TAResult<()> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?
        .into_os_string()
        .into_string()
        .map_err(|_| anyhow::anyhow!("Failed to get path"))?;
    let mut save_data = SaveData::load(Path::new(&path))?;
    save_data.is_notification_exceeded_goal_lap_time = is_notification_exceeded_goal_lap_time;
    SaveData::save(save_data, Path::new(&path))?;
    app.emit("update-setting", ())
        .map_err(|e| anyhow::anyhow!(e))?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn set_is_complete_project(
    app: tauri::AppHandle,
    project_id: uuid::Uuid,
    is_complete: bool,
) -> TAResult<()> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?
        .into_os_string()
        .into_string()
        .map_err(|_| anyhow::anyhow!("Failed to get path"))?;
    let mut save_data = SaveData::load(Path::new(&path))?;
    let project = save_data
        .projects
        .iter_mut()
        .find(|p| p.id == project_id)
        .ok_or(anyhow::anyhow!("Failed to find project"))?;
    project.completed = is_complete;

    SaveData::save(save_data, Path::new(&path))?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn add_project(app: tauri::AppHandle, title: String, deadline: Option<String>) -> TAResult<()> {
    let deadline = deadline
        .and_then(|d| chrono::DateTime::parse_from_str(&d, "%Y-%m-%d %H:%M").ok())
        .map(|dt| dt.with_timezone(&chrono::Utc));
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?
        .into_os_string()
        .into_string()
        .map_err(|_| anyhow::anyhow!("Failed to get path"))?;
    let mut save_data = SaveData::load(Path::new(&path))?;
    save_data.projects.push(Project {
        id: uuid::Uuid::now_v7(),
        title,
        deadline,
        current_elapsed_time: 0,
        completed: false,
        todo_list: TodoList::default(),
    });
    SaveData::save(save_data, Path::new(&path))?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn fetch_project(app: tauri::AppHandle, project_id: uuid::Uuid) -> TAResult<Project> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?
        .into_os_string()
        .into_string()
        .map_err(|_| anyhow::anyhow!("Failed to get path"))?;
    let save_data = SaveData::load(Path::new(&path))?;
    let project = save_data
        .projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or(anyhow::anyhow!("Failed to find project"))?;
    Ok(project.clone())
}

#[tauri::command]
#[specta::specta]
pub fn remove_project(app: tauri::AppHandle, project_id: uuid::Uuid) -> TAResult<()> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?
        .into_os_string()
        .into_string()
        .map_err(|_| anyhow::anyhow!("Failed to get path"))?;
    let mut save_data = SaveData::load(Path::new(&path))?;
    save_data.projects.retain(|p| p.id != project_id);
    SaveData::save(save_data, Path::new(&path))?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn add_todo(app: tauri::AppHandle, project_id: uuid::Uuid) -> TAResult<TodoList> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?
        .into_os_string()
        .into_string()
        .map_err(|_| anyhow::anyhow!("Failed to get path"))?;
    let mut save_data = SaveData::load(Path::new(&path))?;
    let project = save_data
        .projects
        .iter_mut()
        .find(|p| p.id == project_id)
        .ok_or(anyhow::anyhow!("Failed to find project"))?;

    project.todo_list.add_todo();
    let updated_todo_list = project.todo_list.clone();

    SaveData::save(save_data, Path::new(&path))?;

    Ok(updated_todo_list)
}

#[tauri::command]
#[specta::specta]
pub fn remove_todo(app: tauri::AppHandle, project_id: uuid::Uuid) -> TAResult<TodoList> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?
        .into_os_string()
        .into_string()
        .map_err(|_| anyhow::anyhow!("Failed to get path"))?;
    let mut save_data = SaveData::load(Path::new(&path))?;
    let project = save_data
        .projects
        .iter_mut()
        .find(|p| p.id == project_id)
        .ok_or(anyhow::anyhow!("Failed to find project"))?;

    project.todo_list.remove_todo();
    let updated_todo_list = project.todo_list.clone();

    SaveData::save(save_data, Path::new(&path))?;

    Ok(updated_todo_list)
}

#[tauri::command]
#[specta::specta]
pub fn update_todo_item_title(
    app: tauri::AppHandle,
    project_id: uuid::Uuid,
    todo_list: TodoList,
) -> TAResult<()> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?
        .into_os_string()
        .into_string()
        .map_err(|_| anyhow::anyhow!("Failed to get path"))?;
    let mut save_data = SaveData::load(Path::new(&path))?;

    let target_project = save_data
        .projects
        .iter_mut()
        .find(|p| p.id == project_id)
        .ok_or(anyhow::anyhow!("Failed to find project"))?;

    target_project.todo_list = todo_list;

    SaveData::save(save_data, Path::new(&path))?;

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn go_to_next_todo(
    app: tauri::AppHandle,
    project_id: uuid::Uuid,
    parent_id: Option<uuid::Uuid>,
) -> TAResult<TodoList> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?
        .into_os_string()
        .into_string()
        .map_err(|_| anyhow::anyhow!("Failed to get path"))?;
    let mut save_data = SaveData::load(Path::new(&path))?;

    let updated_todo_list =
        if let Some(target_project) = save_data.projects.iter_mut().find(|p| p.id == project_id) {
            target_project.todo_list.go_to_next_todo(parent_id)
        } else {
            return Err(anyhow::anyhow!("Failed to find project").into());
        };

    SaveData::save(save_data, Path::new(&path))?;

    Ok(updated_todo_list)
}

#[tauri::command]
#[specta::specta]
pub fn update_current_elapsed_time(
    app: tauri::AppHandle,
    project_id: uuid::Uuid,
    current_elapsed_time: i32,
) -> TAResult<()> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?
        .into_os_string()
        .into_string()
        .map_err(|_| anyhow::anyhow!("Failed to get path"))?;
    let mut save_data = SaveData::load(Path::new(&path))?;

    let target_project = save_data
        .projects
        .iter_mut()
        .find(|p| p.id == project_id)
        .ok_or(anyhow::anyhow!("Failed to find project"))?;

    target_project.current_elapsed_time = current_elapsed_time;

    SaveData::save(save_data, Path::new(&path))?;

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn get_current_elapsed_time(app: tauri::AppHandle, project_id: uuid::Uuid) -> TAResult<u32> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?
        .into_os_string()
        .into_string()
        .map_err(|_| anyhow::anyhow!("Failed to get path"))?;
    let save_data = SaveData::load(Path::new(&path))?;

    let target_project = save_data
        .projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or(anyhow::anyhow!("Failed to find project"))?;

    Ok(target_project.current_elapsed_time as u32)
}
