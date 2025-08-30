use crate::file;

pub fn get_all() -> String {
    match file::read("example.txt") {
        Ok(content) => content,
        Err(e) => format!("Error reading file: {}", e),
    }
}
