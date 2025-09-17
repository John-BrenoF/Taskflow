use crate::task::{Task, Status, Recurrence};
use anyhow::Result;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use chrono::NaiveDate;
use tauri::Manager;
const FILE_NAME: &str = "tasks.json";

fn get_data_path() -> Result<PathBuf> {
let dir = tauri::api::path::app_dir("taskflow")
.ok_or_else(|| anyhow::anyhow!("couldn't determine app dir"))?;
std::fs::create_dir_all(&dir)?;
Ok(dir.join(FILE_NAME))
}

fn load_tasks() -> Result<Vec<Task>> {
let path = get_data_path()?;
if !path.exists() {
return Ok(Vec::new());
}
let s = fs::read_to_string(path)?;
let v: Vec<Task> = serde_json::from_str(&s)?;
Ok(v)
}

fn save_tasks(tasks: &[Task]) -> Result<()> {
let path = get_data_path()?;
let s = serde_json::to_string_pretty(tasks)?;
fs::write(path, s)?;
Ok(())
}

#[tauri::command]
pub fn list_tasks() -> Result<Vec<Task>, String> {
load_tasks().map_err(|e| e.to_string())
}


#[tauri::command]
pub fn create_task(title: String, description: Option<String>, due_date: Option<String>, category: Option<String>, recurrence: Option<String>) -> Result<Task, String> {
let dd = match due_date {
Some(ds) => Some(NaiveDate::parse_from_str(&ds, "%Y-%m-%d").map_err(|e| e.to_string())?),
None => None,
};
let rec = match recurrence.as_deref() {
Some("daily") => Recurrence::Daily,
Some("weekly") => Recurrence::Weekly,
Some("monthly") => Recurrence::Monthly,
_ => Recurrence::Once,
};
let mut tasks = load_tasks().map_err(|e| e.to_string())?;
let t = Task::new(title, description, dd, category, rec);
tasks.push(t.clone());
save_tasks(&tasks).map_err(|e| e.to_string())?;
Ok(t)
}


#[tauri::command]
pub fn update_task(updated: Task) -> Result<Task, String> {
let mut tasks = load_tasks().map_err(|e| e.to_string())?;
    if let Some(pos) = tasks.iter().position(|t| t.id == updated.id) {
        tasks[pos] = updated.clone();
        save_tasks(&tasks).map_err(|e| e.to_string())?;
        Ok(updated)
    } else {
        Err("Task not found".to_string())
    }
}

#[tauri::command]
pub fn delete_task(id: String) -> Result<(), String> {
    let task_id = uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let mut tasks = load_tasks().map_err(|e| e.to_string())?;
    tasks.retain(|t| t.id != task_id);
    save_tasks(&tasks).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn toggle_status(id: String, status: String) -> Result<Task, String> {
    let task_id = uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let mut tasks = load_tasks().map_err(|e| e.to_string())?;
    if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
        task.status = match status.as_str() {
            "Todo" => Status::Todo,
            "InProgress" => Status::InProgress,
            "Done" => Status::Done,
            _ => return Err("Invalid status".to_string()),
        };
        task.updated_at = chrono::Utc::now();
        save_tasks(&tasks).map_err(|e| e.to_string())?;
        Ok(task.clone())
    } else {
        Err("Task not found".to_string())
    }
}

#[tauri::command]
pub fn export_tasks() -> Result<String, String> {
    let tasks = load_tasks().map_err(|e| e.to_string())?;
    serde_json::to_string(&tasks).map_err(|e| e.to_string())
}