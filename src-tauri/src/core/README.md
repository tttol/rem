# Core Module

This module handles the core business logic of the REM application. It provides basic data structures for tasks and operations for reading and updating them.

## Architecture Overview

The core module consists of the following 3 main components:

```
core/
├── mod.rs          # Module declarations
├── task.rs         # Task data structure definition
├── read_task.rs    # Task reading operations
└── update_task.rs  # Task update operations
```

## Components

### 1. task.rs

Defines the data structure for tasks.

```rust
pub struct Task {
    pub id: String,
    pub title: String,
    pub status: String,
    pub description: String,
}
```

**Features:**
- Serde serialization/deserialization support
- Default status value ("todo") 
- JSON format for persistence

### 2. read_task.rs

Handles task reading operations.

**Main Functions:**

#### `read_all(app_handle: &AppHandle) -> Result<Vec<Task>, tauri::Error>`
- Reads all tasks from the application data directory
- Scans 4 status directories (todo, doing, done, pending)
- Automatically creates directories if they don't exist

#### `read_single(app_handle: &AppHandle, task_id: &str, status: &str) -> Result<Task, tauri::Error>`
- Reads a single task with specified ID and status
- Deserializes from JSON file to Task object

**Internal Functions:**
- `read_tasks_by_status()`: Reads all tasks from specified status directory
- `string_to_task()`: Converts JSON string to Task object

### 3. update_task.rs

Handles task update operations.

**Main Functions:**

#### `update_status(app_handle, task_id, old_status, new_status) -> Result<(), tauri::Error>`
- Changes the status of a task
- Deletes the original file and moves it to the new status directory
- Transactional operation (executes deletion after successful creation)

**Process Flow:**
1. Read existing task data
2. Create Task object with new status
3. Create file in new directory
4. Delete original file

**Internal Functions:**
- `task_to_string()`: Converts Task object to JSON string

## Data Flow

```
Frontend Request
       ↓
   Tauri Command
       ↓
   Core Module
       ↓
  File Operations
       ↓
   JSON Files
```

## File Structure

Tasks are stored in the file system with the following structure:

```
app_data_dir/
├── todo/
│   ├── task1.json
│   └── task2.json
├── doing/
│   └── task3.json
├── done/
│   └── task4.json
└── pending/
    └── task5.json
```

## Error Handling

- File I/O errors: Handled as `tauri::Error::Io`
- JSON parsing errors: Converted from serde errors to tauri errors
- Invalid file formats: Logged and processing skipped

## Dependencies

- `serde`: JSON serialization/deserialization
- `tauri`: Application handle and error types
- `log`: Logging functionality
- `std::fs`: File system operations

## Usage Example

```rust
// Read all tasks
let tasks = read_task::read_all(&app_handle)?;

// Read single task
let task = read_task::read_single(&app_handle, "task123", "todo")?;

// Update task status
update_task::update_status(&app_handle, "task123", "todo", "doing")?;
```