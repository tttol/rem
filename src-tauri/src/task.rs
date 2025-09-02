use std::{fs, path::PathBuf};
use tauri::{AppHandle, Manager};
use log::{info, warn};
use crate::file;


pub fn get_all(app_handle: &AppHandle) -> Result<Vec<String>, tauri::Error> {
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
    info!("todo={:?}, doing={:?}, done={:?}, pending={:?}", todos, doings, dones, pendings);

    let mut all_tasks = Vec::new();
    all_tasks.extend(todos);
    all_tasks.extend(doings);
    all_tasks.extend(dones);
    all_tasks.extend(pendings);

    Ok(all_tasks)
}

const IGNORE_FILES: &[&str] = &[".DS_Store"];

fn get_tasks(target_path: &PathBuf) -> Result<Vec<String>, tauri::Error>{
    let mut tasks = vec![];
    for entry in fs::read_dir(target_path)? {
        let entry = entry?;
        let file_path = entry.path();
        if file_path.is_dir() {
            info!("Skip reading a file because {:?} is a directory.", file_path.file_name());
            continue;
        }
        
        // let file_name = entry
        //     .file_name()
        //     .into_string()
        //     .map_err(|_| tauri::Error::Runtime(tauri_runtime::Error::FailedToSendMessage))?;
        //

        // let file_name = file_path.file_name().or("").into();
        // if IGNORE_FILES.contains(&file_name) { 
        //     info!("{} is ignored.", file_name.as_str());
        //     continue; 
        // }
        let filename = file_path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown");
        if !filename.ends_with(".md") {
            info!("The filename must end with '.md'. Your filename is {}", &filename);
        }
        match file::read_single_file(&file_path) {
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
