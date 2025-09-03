import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect } from "react";
// import "../App.css";
import "../styles.css";
import Lane from "./Lane";
import { Task } from "../dto";
import { Status } from "../enum";
import Header from "./Header";
import { IoReload } from "react-icons/io5";

function App() {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [server_message, setMessage] = useState<string>();

  const updateTaskStatus = (taskId: string, oldStatus: Status, newStatus: Status) => {
    setTasks(prevTasks =>
      prevTasks.map(task =>
        task.id === taskId ? { ...task, status: newStatus } : task
      )
    );
    invoke('update_task_status', {task_id: taskId, old_status: oldStatus, new_status: newStatus}).then(() => console.log('update success'));
  };

  const readTasks = () => {
    invoke<Task[]>('get_all_task').then((fetchedTasks) => {
      setTasks(fetchedTasks);
    }).catch(err => {
      setMessage(`Failed to fetch tasks. {$err}`);
    });
  }

  useEffect(() => {
    reload();
  }, []);

  const reload = () => {
    setTasks([]);
    readTasks();
  }

  return (
    <>
      <Header />
      <p>{server_message}</p>
      <main className="flex justify-center w-full px-2">
        <div onClick={reload}>
          <IoReload />
        </div>
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
