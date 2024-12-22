use anyhow_tauri::TAResult;
use chrono::{serde::ts_seconds_option, DateTime, Utc};
use std::{
    collections::VecDeque,
    fs::{create_dir_all, File, OpenOptions},
    io::BufReader,
    path::Path,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct SaveData {
    pub theme: String,
    #[serde(rename = "isAutoStart")]
    pub is_auto_start: bool,
    #[serde(rename = "isNotificationOfDeadline")]
    pub is_notification_of_deadline: bool,
    #[serde(rename = "isNotificationExceededGoalLapTime")]
    pub is_notification_exceeded_goal_lap_time: bool,
    #[serde(rename = "projects")]
    pub projects: Vec<Project>,
}
impl SaveData {
    pub fn save(save_data: SaveData, app_data_dir: &Path) -> TAResult<File> {
        if !app_data_dir.exists() {
            create_dir_all(app_data_dir).map_err(|e| anyhow::anyhow!(e))?;
        }

        let file_path = Path::new(app_data_dir).join("save_data.json");

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true) // 上書き
            .open(&file_path)
            .map_err(|e| anyhow::anyhow!(e))?;

        serde_json::to_writer(&file, &save_data).map_err(|e| anyhow::anyhow!(e))?;
        Ok(file)
    }
    pub fn load(app_data_dir: &Path) -> TAResult<SaveData> {
        let file = match File::open(Path::new(app_data_dir).join("save_data.json")) {
            Ok(file) => file,
            Err(_) => Self::save(Self::default(), app_data_dir)?,
        };

        let reader = BufReader::new(file);
        let save_data: SaveData =
            serde_json::from_reader(reader).map_err(|e| anyhow::anyhow!(e))?;
        Ok(save_data)
    }
}
impl Default for SaveData {
    fn default() -> Self {
        Self {
            is_auto_start: false,
            is_notification_of_deadline: false,
            is_notification_exceeded_goal_lap_time: false,
            theme: "nord".to_string(),
            projects: Vec::new(),
        }
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub(crate) struct Project {
    pub id: uuid::Uuid,
    pub title: String,
    #[serde(with = "ts_seconds_option")]
    pub deadline: Option<DateTime<Utc>>,
    #[serde(rename = "currentElapsedTime")]
    pub current_elapsed_time: i32,
    pub completed: bool,
    #[serde(rename = "todoList")]
    pub todo_list: TodoList,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type, Default)]
pub(crate) struct TodoList {
    pub checked_todos: Vec<Todo>,
    pub unchecked_todos: VecDeque<Todo>,
}
impl TodoList {
    pub fn add_todo(&mut self) {
        let todo = Todo {
            id: uuid::Uuid::now_v7(),
            title: "新規タスク".to_string(),
            lap_time: None,
            elapsed_time: None,
            branch_name: None,
            sub_todo_list: TodoList::default(),
        };
        self.unchecked_todos.push_back(todo);
    }
    pub fn remove_todo(&mut self) {
        self.unchecked_todos.pop_back();
    }
    pub fn go_to_next_todo(&mut self, parent_todo_id: Option<uuid::Uuid>) -> TodoList {
        if parent_todo_id.is_some() {
            let sub_todo_list = &mut self
                .unchecked_todos
                .front_mut()
                .expect("parent_todo_id not found")
                .sub_todo_list;
            if let Some(todo) = sub_todo_list.unchecked_todos.pop_front() {
                sub_todo_list.checked_todos.push(todo);
            }
        } else if let Some(todo) = self.unchecked_todos.pop_front() {
            self.checked_todos.push(todo);
        }
        self.clone()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub(crate) struct Todo {
    pub id: uuid::Uuid,
    pub title: String,
    #[serde(rename = "lapTime")]
    pub lap_time: Option<i32>,
    #[serde(rename = "elapsedTime")]
    pub elapsed_time: Option<i32>,
    #[serde(rename = "branchName")]
    pub branch_name: Option<String>,
    #[serde(rename = "subTodoList")]
    pub sub_todo_list: TodoList,
}
