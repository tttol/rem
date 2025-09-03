use std::path::PathBuf;

use log::info;
use tauri::{AppHandle, Manager};

pub fn get(app_handle: &AppHandle) -> Result<PathBuf, tauri::Error> {
    let app_data_dir = app_handle.path().app_data_dir().map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, e)
    })?;
    info!("app_data_dir={}", app_data_dir.display());

    return Ok(app_data_dir)
}
