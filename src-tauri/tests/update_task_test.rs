use std::fs;
use tempfile::TempDir;
use rem_lib::core::{task::Task, update_task};

#[test]
fn should_update_title_and_content() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let app_data_dir = temp_dir.path().to_path_buf();
    
    let todo_dir = app_data_dir.join("todo");
    fs::create_dir_all(&todo_dir).expect("Failed to create todo directory");
    
    let original_task = Task {
        id: "test_task_id".to_string(),
        title: "Original Title".to_string(),
        status: "todo".to_string(),
        description: "Original description".to_string(),
    };
    
    let original_task_json = serde_json::to_string(&original_task).expect("Failed to serialize original task");
    let task_file_path = todo_dir.join("test_task_id.json");
    fs::write(&task_file_path, original_task_json).expect("Failed to write original task file");
    
    // Act
    let result = update_task::update_content(
        &app_data_dir,
        "test_task_id",
        "todo",
        "Updated Title",
        "Updated description"
    );
    
    // Assert
    assert!(result.is_ok(), "update_content should succeed");
    
    let updated_content = fs::read_to_string(&task_file_path).expect("Failed to read updated task file");
    let updated_task: Task = serde_json::from_str(&updated_content).expect("Failed to parse updated task");
    
    assert_eq!(updated_task.id, "test_task_id");
    assert_eq!(updated_task.title, "Updated Title");
    assert_eq!(updated_task.status, "todo");
    assert_eq!(updated_task.description, "Updated description");
}
