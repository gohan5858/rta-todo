use crate::save_data::{Project, SaveData, Todo};
use anyhow_tauri::TAResult;
use std::{collections::VecDeque, path::Path};
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
        todo_list: Vec::new(),
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
pub fn add_todo(
    app: tauri::AppHandle,
    project_id: uuid::Uuid,
    title: String,
) -> TAResult<(Vec<Todo>, Vec<Todo>)> {
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

#[tauri::command]
#[specta::specta]
pub fn remove_todo(app: tauri::AppHandle, project_id: uuid::Uuid) -> TAResult<Vec<Todo>> {
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

    if let Some(last_todo) = project.todo_list.last() {
        (!last_todo.checkable && !last_todo.checked).then(|| project.todo_list.pop());
    }

    let unchecked_todo_list: Vec<Todo> = project
        .todo_list
        .clone()
        .into_iter()
        .filter(|t| !t.checked)
        .collect();

    SaveData::save(save_data, Path::new(&path))?;

    Ok(unchecked_todo_list)
}

#[tauri::command]
#[specta::specta]
pub fn update_todo_item_title(
    app: tauri::AppHandle,
    project_id: uuid::Uuid,
    checked_todo_list: Vec<Todo>,
    unchecked_todo_list: Vec<Todo>,
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

    target_project.todo_list = checked_todo_list
        .into_iter()
        .chain(unchecked_todo_list)
        .collect();

    SaveData::save(save_data, Path::new(&path))?;

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn go_to_next_todo(
    app: tauri::AppHandle,
    project_id: uuid::Uuid,
    lap_time: i32,
) -> TAResult<(Vec<Todo>, Vec<Todo>)> {
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

    let (mut unchecked_todo_list, mut checked_todo_list): (VecDeque<Todo>, Vec<Todo>) = {
        let (unchecked, checked): (Vec<Todo>, Vec<Todo>) = target_project
            .todo_list
            .clone()
            .into_iter()
            .partition(|t| !t.checked);
        (unchecked.into(), checked)
    };

    let previous_todo_lap_time = checked_todo_list.last().and_then(|t| t.lap_time);

    let target_todo_elapsed_time = lap_time - previous_todo_lap_time.unwrap_or(0);

    if let Some(mut target_todo) = unchecked_todo_list.pop_front() {
        target_todo.lap_time = Some(lap_time);
        // ミリ秒から分に変換
        let target_todo_elapsed_time_min = target_todo_elapsed_time / 1000 / 60;
        target_todo.elapsed_time = Some(target_todo_elapsed_time_min);
        target_todo.checked = true;
        target_todo.checkable = false;
        checked_todo_list.push(target_todo);
    };

    // popした後なので次のtodoがあれば最初の要素が次のtodo
    if let Some(next_todo) = unchecked_todo_list.front_mut() {
        next_todo.checkable = true;
    }

    target_project.todo_list = checked_todo_list
        .clone()
        .into_iter()
        .chain(unchecked_todo_list.clone().into_iter())
        .collect();

    SaveData::save(save_data, Path::new(&path))?;

    Ok((unchecked_todo_list.into(), checked_todo_list))
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
