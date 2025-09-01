use std::{fs, path::PathBuf};
use tauri::{AppHandle, Manager};
use log::{info};
use crate::file;

pub fn get_all(app_handle: &AppHandle) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let app_data_dir = app_handle.path().app_data_dir().map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, e)
    })?;
    info!("app_data_dir={}", app_data_dir.display());

    // create a directory if it does not exist.
    let todo_path = app_data_dir.join("todo");
    let doing_path = app_data_dir.join("doing");
    let done_path = app_data_dir.join("done");
    let pending_path = app_data_dir.join("pending");
    std::fs::create_dir_all(&todo_path);
    std::fs::create_dir_all(&doing_path);
    std::fs::create_dir_all(&done_path);
    std::fs::create_dir_all(&pending_path);

    let todos = get_tasks(&todo_path)?;
    let doings = get_tasks(&doing_path)?;
    let dones = get_tasks(&done_path)?;
    let pendings = get_tasks(&pending_path)?;

    let mut all_tasks = Vec::new();
    all_tasks.extend(todos);
    all_tasks.extend(doings);
    all_tasks.extend(dones);
    all_tasks.extend(pendings);

    Ok(all_tasks)
}

fn get_tasks(target_path: &PathBuf) -> Result<Vec<String>, Box<dyn std::error::Error>>{
    let mut tasks = vec![];
    for entry in fs::read_dir(target_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            continue;
        }
        
        let file_name = entry
            .file_name()
            .into_string()
            .map_err(|e| format!("Failed to converting into string. {:?}", e))?;
        match file::read_single_file(&file_name, &path) {
            Ok(content) => tasks.push(content),
            Err(e) => eprintln!("Error reading file: {}", e),
        }
    }
    return Ok(tasks);
}

// fn read_file(filename: &str, path: &PathBuf) -> String {
//     match file::read_single_file(filename, path) {
//         Ok(content) => content,
//         Err(e) => format!("Error reading file: {}", e),
//     }
// }
