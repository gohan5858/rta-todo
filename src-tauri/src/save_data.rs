use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::BufReader,
    path::Path,
};

use anyhow_tauri::TAResult;

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
    // HACK: 本来はOption<chrono::DateTime<chrono::Utc>> にしたいが、specta::Type が対応していないため、Option<String> にしている
    pub deadline: Option<String>,
    pub completed: bool,
    #[serde(rename = "todoList")]
    pub todo_list: Vec<Todo>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub(crate) struct Todo {
    pub id: uuid::Uuid,
    pub title: String,
    pub lap_time: Option<i32>,
    pub elapsed_time: Option<i32>,
    pub checked: bool,
    pub checkable: bool,
    pub branch_name: Option<String>,
}
