mod file;
mod task;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn get_all_task(app_handle: tauri::AppHandle) -> Result<Vec<String>, tauri::Error>{
    task::get_all(&app_handle).map_err(|e| tauri::Error::from(e))
}
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_all_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
