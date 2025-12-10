use std::{fs, path::PathBuf};
use log::info;

pub fn delete_all_done_tasks(app_data_dir: &PathBuf) -> Result<(), tauri::Error> {
    info!("Deleting all done tasks");
    let done_dir = app_data_dir.join("done");

    if !done_dir.exists() {
        info!("Done directory does not exist");
        return Ok(());
    }

    for entry in fs::read_dir(done_dir)? {
        let entry = entry?;
        let file_path = entry.path();

        if file_path.is_file() {
            let filename = file_path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown");

            if filename.ends_with(".json") {
                info!("Deleting file: {:?}", file_path);
                fs::remove_file(file_path)?;
            }
        }
    }

    Ok(())
}
