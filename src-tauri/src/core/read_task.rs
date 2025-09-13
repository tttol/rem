use std::{fs, path::PathBuf};
use tauri::{AppHandle, Manager};
use log::{info};
use crate::{core::{task::Task, task_util}, fileio::{app_data_dir, file}};

pub fn read_all(app_data_dir: &PathBuf) -> Result<Vec<Task>, tauri::Error> {
    let mut all_tasks: Vec<Task> = Vec::new();
    
    for status in &["todo", "doing", "done", "pending"] {
        let path = app_data_dir.join(status);

        // create a directory if it does not exist.
        let _ = std::fs::create_dir_all(&path);

        let tasks = read_tasks_by_status(&path)?;
        info!("{}={:?}", &status, tasks);
        all_tasks.extend(tasks);
    }
    info!("all_tasks={:?}", all_tasks);

    Ok(all_tasks)
}

pub fn read_single(app_handle: &AppHandle, task_id: &str, status: &str) -> Result<Task, tauri::Error> {
    let app_data_dir = app_data_dir::get(app_handle)?;
    let file_path = app_data_dir.join(status).join(format!("{}.json", task_id));
    let content = file::read(&file_path)?;
    let task = task_util::string_to_task(&content)?;

    Ok(task)
}

fn read_tasks_by_status(target_path: &PathBuf) -> Result<Vec<Task>, tauri::Error>{
    let mut tasks = vec![];
    let status = target_path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("todo");
    
    for entry in fs::read_dir(target_path)? {
        let entry = entry?;
        let file_path = entry.path();
        if file_path.is_dir() {
            info!("Skip reading a file because {:?} is a directory.", file_path.file_name());
            continue;
        }
        
        let filename = file_path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown");
        if !filename.ends_with(".json") {
            info!("The filename must end with '.json'. Your filename is {}", &filename);
            continue;
        }
        
        
        match file::read(&file_path) {
            Ok(content) => {
                match task_util::string_to_task(&content) {
                    Ok(mut task) => {
                        task.status = status.to_string();
                        tasks.push(task);
                    },
                    Err(e) => eprintln!("Error parsing task: {}", e),
                }
            },
            Err(e) => eprintln!("Error reading file: {}", e),
        }
    }
    Ok(tasks)
}

