import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect } from "react";
import "../styles.css";
import Lane from "./Lane";
import { Task } from "../dto";
import { Status } from "../enum";
import Header from "./Header";
import { IoReload } from "react-icons/io5";
import { IoIosAddCircleOutline } from "react-icons/io";
import { IconContext } from "react-icons";
import CreateForm from "./CreateForm";

function App() {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [server_message, setMessage] = useState<string>();
  const [isShowForm, setIsShowForm] = useState<boolean>(false);

  const updateTaskStatus = (taskId: string, oldStatus: Status, newStatus: Status) => {
    setTasks(prevTasks =>
      prevTasks.map(task =>
        task.id === taskId ? { ...task, status: newStatus } : task
      )
    );
    invoke('update_task_status', {taskId: taskId, oldStatus: oldStatus, newStatus: newStatus})
      .then(() => {
        console.log('update success');
      })
      .catch(err => {
        console.error('update failed:', err);
        setMessage(`Failed to update task. ${err}`);
      });
  };

  const readTasks = () => {
    invoke<Task[]>('get_all_task').then((fetchedTasks) => {
      setTasks(fetchedTasks);
    }).catch(err => {
      setMessage(`Failed to fetch tasks. ${err}`);
    });
  }

  useEffect(() => {
    reload();
  }, []);

  const reload = () => {
    setTasks([]);
    setMessage("");
    readTasks();
  }

  const showForm = () => {
    setIsShowForm(!isShowForm)
  }

  return (
    <>
      <Header />
      <p>{server_message}</p>
      <main className="w-full px-8 text-center">
        <IconContext.Provider value={{size: "2.5em"}}>
          <div className="flex justify-end">
            <div onClick={reload}>
              <IoReload className="w-full"/>Reload
            </div>
            <div className="ml-5" onClick={showForm}>
              <IoIosAddCircleOutline />Add
            </div>
          </div>
        </IconContext.Provider>
        <div className={`transition-all duration-500 ease-in-out overflow-hidden ${
          isShowForm ? 'max-h-96 opacity-100' : 'max-h-0 opacity-0'
        }`}>
          <CreateForm setIsShowForm={setIsShowForm} reload={reload} />
        </div>
        <div className="flex justify-center mx-auto w-full max-w-6xl">
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
