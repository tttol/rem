use std::{fs::File, io::{Read, Write}, path::PathBuf};
use tauri::Manager;
use log::{info, warn};

// Composition over inheritance!
// https://medium.com/comsystoreply/28-days-of-rust-part-2-composition-over-inheritance-cab1b106534a
// https://tyfkda.github.io/blog/2020/09/27/composition-over-inheritance.html

pub fn read_single_file(path: &PathBuf) -> std::io::Result<String> {
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

fn write(data: &str, filename: &str) {
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::ErrorKind;

    // #[test]
    // fn test_read_existing_file() {
    //     let test_content = "test content";
    //     let test_file = "test_read.txt";
    //     
    //     fs::write(test_file, test_content).unwrap();
    //     
    //     let result = read(test_file);
    //     
    //     assert!(result.is_ok());
    //     assert_eq!(result.unwrap(), "test content");
    //     
    //     fs::remove_file(test_file).unwrap();
    // }
    //
    // #[test]
    // fn test_read_nonexistent_file() {
    //     let result = read("nonexistent_file.txt");
    //     
    //     assert!(result.is_err());
    //     assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
    // }

    #[test]
    fn test_create_new_file() {
        let test_data = "test file content";
        let filename = "test_create.txt";
        let path = ".";
        let full_path = format!("{}/{}", path, filename);
        
        if fs::metadata(&full_path).is_ok() {
            fs::remove_file(&full_path).unwrap();
        }
        
        let result = create(test_data, filename, path);
        
        assert!(result.is_ok());
        
        let content = fs::read_to_string(&full_path).unwrap();
        assert_eq!(content, test_data);
        
        fs::remove_file(&full_path).unwrap();
    }

    #[test]
    fn test_create_file_already_exists() {
        let test_data = "existing content";
        let filename = "test_existing.txt";
        let path = ".";
        let full_path = format!("{}/{}", path, filename);
        
        fs::write(&full_path, "original content").unwrap();
        
        let result = create(test_data, filename, path);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), ErrorKind::AlreadyExists);
        
        fs::remove_file(&full_path).unwrap();
    }

    #[test]
    fn test_create_invalid_path() {
        let result = create("test", "test.txt", "/invalid/path/that/does/not/exist");
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
    }
}


