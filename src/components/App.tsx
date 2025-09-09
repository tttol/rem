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
  const [fadingTasks, setFadingTasks] = useState<Set<string>>(new Set());
  const [fadingInTasks, setFadingInTasks] = useState<Set<string>>(new Set());

  const updateTaskStatus = (taskId: string, oldStatus: Status, newStatus: Status) => {
    // Start fade out animation
    setFadingTasks(prev => new Set([...prev, taskId]));
    
    // Update task status after fade animation
    setTimeout(() => {
      setTasks(prevTasks =>
        prevTasks.map(task =>
          task.id === taskId ? { ...task, status: newStatus } : task
        )
      );
      // Remove from fading tasks and start fade in
      setFadingTasks(prev => {
        const newSet = new Set(prev);
        newSet.delete(taskId);
        return newSet;
      });
      setFadingInTasks(prev => new Set([...prev, taskId]));
      
      // Remove from fading in tasks after animation
      setTimeout(() => {
        setFadingInTasks(prev => {
          const newSet = new Set(prev);
          newSet.delete(taskId);
          return newSet;
        });
      }, 300);
    }, 300);
    
    invoke('update_task_status', {taskId: taskId, oldStatus: oldStatus, newStatus: newStatus})
      .then(() => {
        console.log('update success');
      })
      .catch(err => {
        console.error('update failed:', err);
        setMessage(`Failed to update task. ${err}`);
        // Revert the status change and remove from fading if the update fails
        setFadingTasks(prev => {
          const newSet = new Set(prev);
          newSet.delete(taskId);
          return newSet;
        });
        setFadingInTasks(prev => {
          const newSet = new Set(prev);
          newSet.delete(taskId);
          return newSet;
        });
        setTasks(prevTasks =>
          prevTasks.map(task =>
            task.id === taskId ? { ...task, status: oldStatus } : task
          )
        );
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
    setIsShowForm(false);
    setTasks([]);
    setMessage("");
    readTasks();
  }

  const showForm = () => {
    setIsShowForm(!isShowForm)
  }

  return (
    <div className="min-h-screen bg-gray-900 text-white">
      <Header />
      <p className="text-red-400">{server_message}</p>
      <main className="w-full px-8 text-center">
        <IconContext.Provider value={{size: "2.5em"}}>
          <div className="flex justify-end text-gray-300">
            <div onClick={reload} className="cursor-pointer hover:text-white transition-colors">
              <IoReload className="w-full"/>Reload
            </div>
            <div className="ml-5 cursor-pointer hover:text-white transition-colors" onClick={showForm}>
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
          <Lane tasks={tasks} statusLabel={Status.TODO} updateTaskStatus={updateTaskStatus} reload={reload} fadingTasks={fadingTasks} fadingInTasks={fadingInTasks} />
          <Lane tasks={tasks} statusLabel={Status.DOING} updateTaskStatus={updateTaskStatus} reload={reload} fadingTasks={fadingTasks} fadingInTasks={fadingInTasks} />
          <Lane tasks={tasks} statusLabel={Status.DONE} updateTaskStatus={updateTaskStatus} reload={reload} fadingTasks={fadingTasks} fadingInTasks={fadingInTasks} />
        </div>
      </main>
    </div>
  );
}

export default App;
