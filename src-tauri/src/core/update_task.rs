use std::path;

use crate::{core::{read_task, task::Task, task_util}, fileio::{self, file}};
use fileio::app_data_dir;
use log::info;
use tauri::AppHandle;

pub fn update_content(app_handle: &AppHandle, task_id: &str, status: &str, title: &str, description: &str) -> Result<(), tauri::Error> {
    let app_data_dir = app_data_dir::get(app_handle)?;
    let modified_data = Task {
        id: task_id.to_string(),
        title: title.to_string(),
        status: status.to_string(),
        description: description.to_string()
    };
    let task_string = task_util::task_to_string(&modified_data)?;
    
    info!("updated content data={:?}", modified_data);
    file::create(&task_string, &app_data_dir.join(&status).join(format!("{}.json", task_id)))
        .map_err(|e| tauri::Error::Io(e))
}


pub fn update_status(app_handle: &AppHandle, task_id: &str, old_status: &str, new_status: &str) -> Result<(), tauri::Error> {
    let app_data_dir = app_data_dir::get(app_handle)?;
    let original_data = read_task::read_single(app_handle, task_id, old_status)?;
    let modified_data = Task {
        id: task_id.to_string(),
        title: original_data.title,
        status: new_status.to_string(),
        description: original_data.description
    };
    let task_string = task_util::task_to_string(&modified_data)
        .map_err(|e| tauri::Error::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;
    
    info!("updated data={:?}", modified_data);
    match file::create(&task_string, &app_data_dir.join(&new_status).join(format!("{}.json", task_id))) {
        Ok(_) => {
            info!("{:?} has been created.", &app_data_dir.join(&new_status));
            match file::delete(&app_data_dir.join(old_status).join(format!("{}.json", task_id))) {
                Ok(_) => {
                    info!("{:?} has been deleted.", &app_data_dir.join(old_status));
                    Ok(())
                },
                Err(e) => Err(tauri::Error::Io(e))
            }
        },
        Err(e) => Err(tauri::Error::Io(e))
    }
}

