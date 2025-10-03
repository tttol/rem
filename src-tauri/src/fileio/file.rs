use std::{fs::File, io::{Read, Write}, path::PathBuf};
use log::{info};

pub fn read(path: &PathBuf) -> std::io::Result<String> {
    info!("Reading the {:?}", &path);
    let mut f = File::open(path)?;
    let mut data = String::new();
    f.read_to_string(&mut data)?;
    Ok(data)
}

pub fn delete(path: &PathBuf) -> std::io::Result<()> {
    info!("Deleting a file: {:?}", path);
    std::fs::remove_file(path)?;
    Ok(())
}

pub fn create(data: &str, path: &PathBuf) -> std::io::Result<()> {
    info!("Creating a new file. {:?}", &path);
    let mut f = File::create_new(path)?;
    f.write_all(data.as_bytes())?;
    Ok(())
}

pub fn write(data: &str, path: &PathBuf) -> std::io::Result<()> {
    info!("Write to {:?}", &path);
    let mut f = File::create(path)?;
    f.write_all(data.as_bytes())?;
    Ok(())
}

