use std::{fmt::format, path::PathBuf};

use chrono::Local;

use crate::{core::{task::Task, task_util}, fileio::file};

pub fn create(app_data_dir: &PathBuf, title: &str, description: &str) -> Result<(), tauri::Error> {
    let now = Local::now();
    let timestamp = now.format("%Y%m%d%H%M").to_string();
    let task_id = format!("{}_{}", timestamp, generate_blake3_hash(timestamp.as_bytes()));
    let task = Task {
        id: task_id.to_string(),
        title: title.to_string(),
        status: "todo".to_string(),
        description: description.to_string()
    };

    let task_json = task_util::task_to_string(&task)?;
    file::create(&task_json, &app_data_dir.join(format!("todo/{}.json", &task_id)))
        .map_err(|e| tauri::Error::Anyhow(e.into()))
}


fn generate_blake3_hash(bytes: &[u8]) -> String {
    let hash = blake3::hash(bytes);
    let bytes = hash.as_bytes();
    hex::encode(&bytes[..32])
}
