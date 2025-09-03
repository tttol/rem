use crate::{core::{read_task, task::Task}, file, fileio};
use fileio::app_data_dir;
use tauri::AppHandle;

pub fn update() {

}


pub fn update_status(app_handle: &AppHandle, task_id: &str, old_status: &str, new_status: &str) -> Result<(), tauri::Error> {
    let app_data_dir = app_data_dir::get(app_handle)?;
    let original_data = read_task::read_single(app_handle, task_id, old_status)?;
    let modified_data = Task {
        id: task_id,
        title: original_data.title,
        status: new_status,
        description: original_data.description
    };
    file::create(&app_data_dir.join(new_status));
    file::delete(app_data_dir.join(old_status));
    Ok(())
}

fn task_to_string(task: &Task) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(task)
}
