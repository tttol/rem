use std::path::PathBuf;

use log::info;
use tauri::{AppHandle, Manager};

pub fn get(app_handle: &AppHandle) -> Result<PathBuf, tauri::Error> {
    let base_dir = if cfg!(debug_assertions) {
        // Development environment: use a dev-specific directory
        let current_dir = std::env::current_dir().map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, e)
        })?;
        current_dir.join("dev_data")
    } else {
        // Production environment: use standard app data directory
        app_handle.path().app_data_dir().map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, e)
        })?
    };
    
    info!("app_data_dir={}", base_dir.display());
    
    // Create the directory if it doesn't exist
    std::fs::create_dir_all(&base_dir).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to create directory: {}", e))
    })?;
    
    Ok(base_dir)
}
