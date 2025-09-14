use std::fs;
use tempfile::TempDir;
use rem_lib::core::{task::Task, read_task};

#[test]
fn read_all_task_from_json_files() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();
    
    let todo_dir = temp_path.join("todo");
    let doing_dir = temp_path.join("doing");
    let done_dir = temp_path.join("done");
    
    fs::create_dir_all(&todo_dir).expect("Failed to create todo directory");
    fs::create_dir_all(&doing_dir).expect("Failed to create doing directory");
    fs::create_dir_all(&done_dir).expect("Failed to create done directory");
    
    let task1 = Task {
        id: "task1".to_string(),
        title: "First Task".to_string(),
        status: "todo".to_string(),
        description: "Description for first task".to_string(),
    };
    
    let task2 = Task {
        id: "task2".to_string(),
        title: "Second Task".to_string(),
        status: "doing".to_string(),
        description: "Description for second task".to_string(),
    };
    
    let task3 = Task {
        id: "task3".to_string(),
        title: "Third Task".to_string(),
        status: "done".to_string(),
        description: "Description for third task".to_string(),
    };
    
    let task1_json = serde_json::to_string(&task1).expect("Failed to serialize task1");
    let task2_json = serde_json::to_string(&task2).expect("Failed to serialize task2");
    let task3_json = serde_json::to_string(&task3).expect("Failed to serialize task3");
    
    fs::write(todo_dir.join("task1.json"), task1_json).expect("Failed to write task1.json");
    fs::write(doing_dir.join("task2.json"), task2_json).expect("Failed to write task2.json");
    fs::write(done_dir.join("task3.json"), task3_json).expect("Failed to write task3.json");
    
    // Also create pending directory for complete test
    let pending_dir = temp_path.join("pending");
    fs::create_dir_all(&pending_dir).expect("Failed to create pending directory");
    
    let app_data_dir = temp_path.to_path_buf();
    
    // Act
    let all_tasks = read_task::read_all(&app_data_dir).expect("Failed to read all tasks");
    
    // Assert
    assert_eq!(all_tasks.len(), 3);
    
    let todo_task = all_tasks.iter().find(|t| t.id == "task1").expect("task1 not found");
    assert_eq!(todo_task.title, "First Task");
    assert_eq!(todo_task.status, "todo");
    assert_eq!(todo_task.description, "Description for first task");
    
    let doing_task = all_tasks.iter().find(|t| t.id == "task2").expect("task2 not found");
    assert_eq!(doing_task.title, "Second Task");
    assert_eq!(doing_task.status, "doing");
    
    let done_task = all_tasks.iter().find(|t| t.id == "task3").expect("task3 not found");
    assert_eq!(done_task.title, "Third Task");
    assert_eq!(done_task.status, "done");
}
