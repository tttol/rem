use crate::{core::read_task, file, fileio};
use fileio::app_data_dir;
use tauri::AppHandle;

pub fn update() {

}

pub fn update_status(app_handle: &AppHandle, task_id: &str, old_status: &str, new_status: &str) -> std::io::Result<()> {
    let app_data_dir = app_data_dir::get(app_handle)?;
    let target_data = read_task::read_all(app_handle)
    file::create(&app_data_dir.join(new_status));
    file::delete(app_data_dir.join(old_status));
}

