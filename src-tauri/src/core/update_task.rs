use std::path::PathBuf;

use crate::{core::{task::Task, task_util}, fileio::file};
use log::info;

pub fn update_content(app_data_dir: &PathBuf, task_id: &str, status: &str, title: &str, description: &str) -> Result<(), tauri::Error> {
    let modified_data = Task {
        id: task_id.to_string(),
        title: title.to_string(),
        status: status.to_string(),
        description: description.to_string()
    };
    let task_string = task_util::task_to_string(&modified_data)?;
    
    info!("updated content data={:?}", modified_data);
    file::write(&task_string, &app_data_dir.join(&status).join(format!("{}.json", task_id)))
        .map_err(|e| tauri::Error::Io(e))
}


pub fn update_status(app_data_dir: &PathBuf, task_id: &str, old_status: &str, new_status: &str) -> Result<(), tauri::Error> {
    let file_path = app_data_dir.join(old_status).join(format!("{}.json", task_id));
    let content = file::read(&file_path)?;
    let original_data = task_util::string_to_task(&content)?;
    
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

