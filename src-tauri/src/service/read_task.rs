use std::{fs, path::PathBuf};
use tauri::{AppHandle, Manager};
use log::{info, warn};
use crate::{file, service::task::Task};



pub fn read_all(app_handle: &AppHandle) -> Result<Vec<Task>, tauri::Error> {
    let app_data_dir = app_handle.path().app_data_dir().map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, e)
    })?;
    info!("app_data_dir={}", app_data_dir.display());

    let mut all_tasks: Vec<Task> = Vec::new();
    
    for status in &["todo", "doing", "done", "pending"] {
        let path = app_data_dir.join(status);

        // create a directory if it does not exist.
        std::fs::create_dir_all(&path);

        let tasks = read_tasks_by_status(&path)?;
        info!("{}={:?}", &status, tasks);
        all_tasks.extend(tasks);
    }

    Ok(all_tasks)
}

fn read_tasks_by_status(target_path: &PathBuf) -> Result<Vec<Task>, tauri::Error>{
    let mut tasks = vec![];
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
        if !filename.ends_with(".md") {
            info!("The filename must end with '.md'. Your filename is {}", &filename);
            continue;
        }
        
        
        match file::read_single_file(&file_path) {
            Ok(content) => {
                let task = string_to_task(&content, &filename, &target_path);
                tasks.push(task);
            },
            Err(e) => eprintln!("Error reading file: {}", e),
        }
    }
    return Ok(tasks);
}

fn string_to_task(content: &str, filename: &str, target_path: &PathBuf) -> Task {
    let status = match target_path.file_name().and_then(|name| name.to_str()) {
        Some("todo") => "TODO",
        Some("doing") => "DOING", 
        Some("done") => "DONE",
        Some("pending") => "PENDING",
        _ => "UNKNOWN"
    };
    let lines: Vec<&str> = content.lines().collect();
    let title = lines.first().unwrap_or(&"").to_string();
    let description = if lines.len() > 1 {
        lines[1..].join("\n")
    } else {
        String::new()
    };
    
    return Task {
        id: filename.replace(".md", ""),
        title,
        status: status.to_string(),
        description,
    };
}

