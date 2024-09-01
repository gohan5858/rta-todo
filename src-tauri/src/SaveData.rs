use std::{fs::File, io::BufReader};

use anyhow_tauri::TAResult;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct SaveData {
    #[serde(rename = "todoLists")]
    todo_lists: Vec<TodoList>,
}
impl SaveData {
    pub fn load(path: &str) -> TAResult<SaveData> {
        let file = File::open(path).map_err(|e| anyhow::anyhow!(e))?;
        let reader = BufReader::new(file);
        let save_data: SaveData =
            serde_json::from_reader(reader).map_err(|e| anyhow::anyhow!(e))?;
        Ok(save_data)
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
struct TodoList {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub todos: Vec<Todo>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
struct Todo {
    pub id: i32,
    pub lap_time: Option<i32>,
    pub elapsed_time: Option<i32>,
    pub checked: bool,
    pub checkable: bool,
    pub branch_name: Option<String>,
}
