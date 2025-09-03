mod file;
mod service;
mod fileio;
use service::read_task;
use service::update_task;
use service::task;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn get_all_task(app_handle: tauri::AppHandle) -> Result<Vec<task::Task>, tauri::Error>{
    read_task::read_all(&app_handle).map_err(|e| tauri::Error::from(e))
}

#[tauri::command]
fn update_task_status(app_handle: tauri::AppHandle, task_id: &str, new_status: &str) -> Result<(), tauri::Error>{
    update_task::update_status(app_handle, task_id, new_status);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_all_task, update_task_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
