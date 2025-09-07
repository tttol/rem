use crate::core::task::Task;

pub fn string_to_task(content: &str) -> Result<Task, tauri::Error> {
    let task = serde_json::from_str(content)?;
    Ok(task)
}

pub fn task_to_string(task: &Task) -> Result<String, tauri::Error> {
    serde_json::to_string_pretty(task).map_err(|e| tauri::Error::Anyhow(e.into()))
}

