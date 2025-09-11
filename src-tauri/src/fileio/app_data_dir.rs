use std::path::PathBuf;

use log::info;
use tauri::{AppHandle, Manager};

pub fn get(app_handle: &AppHandle) -> Result<PathBuf, tauri::Error> {
    let base_dir = if cfg!(debug_assertions) {
        // Development environment: use ~/rem/tmp/ directory
        let home_dir = std::env::var("HOME").map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, e)
        })?;
        PathBuf::from(home_dir).join("rem").join("tmp")
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
