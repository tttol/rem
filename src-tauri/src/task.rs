use std::{fs, path::PathBuf};
use tauri::{AppHandle, Manager};
use log::{info, warn};
use crate::file;

pub fn get_all(app_handle: &AppHandle) -> Result<String, Box<dyn std::error::Error>> {
    let app_data_dir = app_handle.path().app_data_dir().map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, e)
    })?;
    info!("app_data_dir={}", app_data_dir.display());

    // create a directory if it does not exist.
    std::fs::create_dir_all(&app_data_dir.join("todo"))?;
    std::fs::create_dir_all(&app_data_dir.join("doing"))?;
    std::fs::create_dir_all(&app_data_dir.join("done"))?;
    std::fs::create_dir_all(&app_data_dir.join("pending"))?;
    
}

fn get_tasks(target_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>>{
    let mut tasks = vec![];
    for entry in fs::read_dir(target_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            continue;
        }
        // let file_name_result = entry.file_name().into_string()?;
        // match file_name_result {
        //     Ok(file_name) => read_file(&file_name, target_path),
        //     Err(e) => format!("Error converting filename: {}", e)
        // }

        // let file_name = entry.file_name().into_string()?;
        
        let file_name = entry.file_name().into_string()
                        .map_err(|e| format!("Failed to converting into string. {}", e))?;
        match file::read_single_file(&file_name, &path) {
            Ok(content) => tasks.push(content),
            Err(e) => format!("Error reading file: {}", e),
        }
    }
    return tasks;
}

fn read_file(filename: &str, path: &PathBuf) -> String {
    match file::read_single_file(filename, path) {
        Ok(content) => content,
        Err(e) => format!("Error reading file: {}", e),
    }
}
