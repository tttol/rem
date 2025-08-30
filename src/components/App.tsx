import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect } from "react";
// import "../App.css";
import "../styles.css";
import Lane from "./Lane";
import { Task } from "../dto";
import { Status } from "../enum";
import Header from "./Header";

function App() {
  const [tasks, setTasks] = useState<Task[]>([
    new Task("1", "Sample Task 1", Status.TODO, "This is a sample todo task"),
    new Task("2", "Sample Task 2", Status.DOING, "This is a sample doing task"),
    new Task("3", "Sample Task 3", Status.DONE, "This is a sample done task"),
    new Task("4", "Sample Task 4", Status.DONE, "This is a sample done task"),
    new Task("5", "Sample Task 5", Status.DONE, "This is a sample done task"),
    new Task("6", "Sample Task 6", Status.DONE, "This is a sample done task"),
  ]);

  const updateTaskStatus = (taskId: string, newStatus: Status) => {
    setTasks(prevTasks =>
      prevTasks.map(task =>
        task.id === taskId ? { ...task, status: newStatus } : task
      )
    );
  };

  useEffect(() => {
    console.log("App started");
    invoke('get_all_task').then((message) => console.log(message));
  }, []);

  return (
    <>
      <Header />
      <main className="flex justify-center w-full px-2">
        <div className="flex">
          <Lane tasks={tasks} statusLabel={Status.TODO} updateTaskStatus={updateTaskStatus} />
          <Lane tasks={tasks} statusLabel={Status.DOING} updateTaskStatus={updateTaskStatus} />
          <Lane tasks={tasks} statusLabel={Status.PENDING} updateTaskStatus={updateTaskStatus} />
          <Lane tasks={tasks} statusLabel={Status.DONE} updateTaskStatus={updateTaskStatus} />
        </div>
      </main>
    </>
  );
}

export default App;
