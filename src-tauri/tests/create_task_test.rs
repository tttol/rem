use std::fs;
use tempfile::TempDir;
use rem_lib::core::{create_task, task::Task};

#[test]
fn create_a_new_task() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let app_data_dir = temp_dir.path().to_path_buf();
    
    let todo_dir = app_data_dir.join("todo");
    fs::create_dir_all(&todo_dir).expect("Failed to create todo directory");
    
    let title = "Test Task";
    let description = "This is a test task description";
    
    // Act
    let result = create_task::create(&app_data_dir, title, description, "todo");
    
    // Assert
    assert!(result.is_ok(), "create_task should succeed");
    
    let todo_files = fs::read_dir(&todo_dir).expect("Failed to read todo directory");
    let json_files: Vec<_> = todo_files
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()? == "json" {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    
    assert_eq!(json_files.len(), 1, "Should create exactly one JSON file");
    
    let json_content = fs::read_to_string(&json_files[0]).expect("Failed to read JSON file");
    let task: Task = serde_json::from_str(&json_content).expect("Failed to parse task");
    
    assert_eq!(task.title, title);
    assert_eq!(task.status, "todo");
    assert_eq!(task.description, description);
    assert!(!task.id.is_empty(), "Task ID should not be empty");
    assert!(task.id.contains("_"), "Task ID should contain timestamp and hash");
}

#[test]
fn create_a_new_task_with_empty_description_and_doing_status() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let app_data_dir = temp_dir.path().to_path_buf();
    
    let todo_dir = app_data_dir.join("doing");
    fs::create_dir_all(&todo_dir).expect("Failed to create todo directory");
    
    let title = "Test Task";
    let description = "";
    
    // Act
    let result = create_task::create(&app_data_dir, title, description, "doing");
    
    // Assert
    assert!(result.is_ok(), "create_task should succeed even with empty description");
    
    let todo_files = fs::read_dir(&todo_dir).expect("Failed to read todo directory");
    let json_files: Vec<_> = todo_files
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()? == "json" {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    
    assert_eq!(json_files.len(), 1, "Should create exactly one JSON file");
    
    let json_content = fs::read_to_string(&json_files[0]).expect("Failed to read JSON file");
    let task: Task = serde_json::from_str(&json_content).expect("Failed to parse task");
    
    assert_eq!(task.title, title);
    assert_eq!(task.description, "");
    assert_eq!(task.status, "doing");
}

