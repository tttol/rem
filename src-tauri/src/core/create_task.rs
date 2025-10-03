use std::{path::PathBuf};

use chrono::Local;
use log::info;

use crate::{core::{task::Task, task_util}, fileio::file};

pub fn create(app_data_dir: &PathBuf, title: &str, description: &str) -> Result<(), tauri::Error> {
    let task_id = generate_blake3_hash_id();
    info!("task_id={}", &task_id);
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

fn generate_blake3_hash_id() -> String {
    let now = Local::now();
    let timestamp = now.format("%Y%m%d%H%M%S%3f").to_string();
    let hash = blake3::hash(timestamp.as_bytes());
    let bytes = hash.as_bytes();

    format!("{}_{}", timestamp, hex::encode(&bytes[..32]))

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_blake3_hash_id_format() {
        let id = generate_blake3_hash_id();
        let parts: Vec<&str> = id.split('_').collect();

        assert_eq!(parts.len(), 2);

        let timestamp = parts[0];
        assert_eq!(timestamp.len(), 17);
        assert!(timestamp.chars().all(|c| c.is_numeric()));

        let hash = parts[1];
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_generate_blake3_hash_id_uniqueness() {
        let id1 = generate_blake3_hash_id();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let id2 = generate_blake3_hash_id();

        assert_ne!(id1, id2);
    }
}

