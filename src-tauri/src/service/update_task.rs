use crate::{file, fileio};
use fileio::app_data_dir;
use tauri::AppHandle;

pub fn update() {

}

pub fn update_status(app_handle: &AppHandle, task_id: &str, old_status: &str, new_status: &str) {
    let app_data_dir = app_data_dir::get(app_handle)?;
    file::delete(app_data_dir.join(old_status));
    file::create_new(app_data_dir.join(new_status));
}

