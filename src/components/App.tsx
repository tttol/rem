import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect } from "react";
// import "../App.css";
import "../styles.css";
import Lane from "./Lane";
import { Task } from "../dto";
import { Status } from "../enum";
import Header from "./Header";

function App() {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [message, setMessage] = useState<string>();

  const updateTaskStatus = (taskId: string, newStatus: Status) => {
    setTasks(prevTasks =>
      prevTasks.map(task =>
        task.id === taskId ? { ...task, status: newStatus } : task
      )
    );
  };

  useEffect(() => {
    console.log("App started");
    invoke<Task[]>('get_all_task').then((fetchedTasks) => {
      setTasks(fetchedTasks);
      setMessage(`Loaded ${fetchedTasks.length} tasks`);
    }).catch(err => {
      console.error("Failed to fetch tasks:", err);
      setMessage("Failed to load tasks");
    });
  }, []);

  return (
    <>
      <Header />
      <p>{message}</p>
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
