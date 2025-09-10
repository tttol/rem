use crate::core::task::Task;

pub fn string_to_task(content: &str) -> Result<Task, tauri::Error> {
    let task = serde_json::from_str(content)?;
    Ok(task)
}

pub fn task_to_string(task: &Task) -> Result<String, tauri::Error> {
    serde_json::to_string_pretty(task).map_err(|e| tauri::Error::Anyhow(e.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_task_valid_json() {
        let json = r#"{
            "id": "test-id",
            "title": "Test Task",
            "status": "todo",
            "description": "Test description"
        }"#;
        
        let result = string_to_task(json);
        assert!(result.is_ok());
        
        let task = result.unwrap();
        assert_eq!(task.id, "test-id");
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.status, "todo");
        assert_eq!(task.description, "Test description");
    }

    #[test]
    fn test_string_to_task_default_status() {
        let json = r#"{
            "id": "test-id",
            "title": "Test Task",
            "description": "Test description"
        }"#;
        
        let result = string_to_task(json);
        assert!(result.is_ok());
        
        let task = result.unwrap();
        assert_eq!(task.status, "todo");
    }

    #[test]
    fn test_string_to_task_invalid_json() {
        let invalid_json = "invalid json";
        let result = string_to_task(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_task_to_string() {
        let task = Task {
            id: "test-id".to_string(),
            title: "Test Task".to_string(),
            status: "doing".to_string(),
            description: "Test description".to_string(),
        };
        let expected = r#"{
  "id": "test-id",
  "title": "Test Task",
  "status": "doing",
  "description": "Test description"
}"#;

        let actual = task_to_string(&task);
        assert!(actual.is_ok());
        

        let actual_string = actual.unwrap();
        assert_eq!(actual_string, expected);
    }
}

