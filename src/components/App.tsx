import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
// import "../App.css";
import Lane from "./Lane";
import { Task } from "../dto";
import { Status } from "../enum";

function App() {
  const [tasks, setTasks] = useState<Task[]>([
    new Task(1, "Sample Task 1", Status.TODO, "This is a sample todo task"),
    new Task(2, "Sample Task 2", Status.DOING, "This is a sample doing task"),
    new Task(3, "Sample Task 3", Status.DONE, "This is a sample done task")
  ]);

  const updateTaskStatus = (taskId: number, newStatus: Status) => {
    setTasks(prevTasks =>
      prevTasks.map(task =>
        task.id === taskId ? { ...task, status: newStatus } : task
      )
    );
  };

  return (
    <main className="container">
      <Lane tasks={tasks} statusLabel={Status.TODO} updateTaskStatus={updateTaskStatus} />
      <Lane tasks={tasks} statusLabel={Status.DOING} updateTaskStatus={updateTaskStatus} />
      <Lane tasks={tasks} statusLabel={Status.PENDING} updateTaskStatus={updateTaskStatus} />
      <Lane tasks={tasks} statusLabel={Status.DONE} updateTaskStatus={updateTaskStatus} />
    </main>
  );
}

export default App;
