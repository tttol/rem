pub mod core;
pub mod fileio;
use core::read_task;
use core::update_task;
use core::create_task;
use core::task;

use log::info;

use crate::fileio::app_data_dir;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn get_all_task(app_handle: tauri::AppHandle) -> Result<Vec<task::Task>, tauri::Error>{
    info!("Calling get_all_task");
    let app_data_dir = app_data_dir::get(&app_handle)?;
    read_task::read_all(&app_data_dir).map_err(|e| tauri::Error::from(e))
}

#[tauri::command]
fn create_task(app_handle: tauri::AppHandle, title: &str, description: &str) -> Result<(), tauri::Error> {
    info!("Calling create_task");
    let app_data_dir = app_data_dir::get(&app_handle)?;
    create_task::create(&app_data_dir, &title, &description)
}

#[tauri::command]
fn update_task_status(app_handle: tauri::AppHandle, task_id: &str, old_status: &str, new_status: &str) -> Result<(), tauri::Error>{
    info!("Calling update_task_status");
    let app_data_dir = app_data_dir::get(&app_handle)?;
    update_task::update_status(&app_data_dir, task_id, old_status, new_status)
}

#[tauri::command]
fn update_task_content(app_handle: tauri::AppHandle, task_id: &str, status: &str, title: &str, description: &str) -> Result<(), tauri::Error> {
    info!("Calling update_task_content");
    let app_data_dir = app_data_dir::get(&app_handle)?;
    update_task::update_content(&app_data_dir, task_id, status, title, description)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_all_task, update_task_status, create_task, update_task_content])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
