import { FaPause, FaPlay, FaEdit } from "react-icons/fa";
import { Task } from "../dto";
import { Status } from "../enum";
import { MdDone } from "react-icons/md";
import { ImArrowLeft2 } from "react-icons/im";
import { useState } from "react";
import { useForm, SubmitHandler } from "react-hook-form";
import { invoke } from "@tauri-apps/api/core";

type EditInputs = {
  title: string;
  description: string;
};

function Item({ task, updateTaskStatus, reload }: { task: Task; updateTaskStatus: (taskId: string, oldStatus: Status, newStatus: Status) => void; reload: () => void }) {
  const [isEditing, setIsEditing] = useState(false);
  const {
    register,
    handleSubmit,
    formState: { errors },
    reset
  } = useForm<EditInputs>();

  const onSubmit: SubmitHandler<EditInputs> = (data) => {
    invoke('update_task_content', {
      taskId: task.id,
      status: task.status,
      title: data.title,
      description: data.description
    })
      .then(() => {
        console.log('update content success');
        setIsEditing(false);
        reset();
        reload();
      })
      .catch(err => {
        console.error('update content failed:', err);
      });
  };

  const handleEditClick = () => {
    setIsEditing(true);
    reset({
      title: task.title,
      description: task.description
    });
  };
  
  const isCompleted = task.status === Status.DONE;

  return (
    <div className="border border-gray-600 bg-gray-800 p-2 m-1 text-white transition-all duration-300 ease-in-out">
      <div className="text-left">
        {isEditing ? (
          <form onSubmit={handleSubmit(onSubmit)} className="space-y-2">
            <div>
              <input
                type="text"
                autoCapitalize="off"
                className={`w-full px-2 py-1 border rounded text-sm bg-gray-700 text-white ${errors.title ? 'border-red-500' : 'border-gray-500'}`}
                {...register("title", { required: "Title is required" })}
              />
              {errors.title && <p className="text-red-500 text-xs">{errors.title.message}</p>}
            </div>
            <div>
              <textarea
                rows={3}
                autoCapitalize="off"
                className="w-full px-2 py-1 border border-gray-500 bg-gray-700 text-white rounded text-sm resize-vertical"
                {...register("description")}
              />
            </div>
            <div className="flex space-x-2">
              <button
                type="submit"
                className="px-3 py-1 bg-blue-500 text-white rounded text-sm hover:bg-blue-600"
              >
                Save
              </button>
              <button
                type="button"
                onClick={() => setIsEditing(false)}
                className="px-3 py-1 bg-gray-500 text-white rounded text-sm hover:bg-gray-600"
              >
                Cancel
              </button>
            </div>
          </form>
        ) : (
          <>
            <div className="flex justify-between items-center">
              <p className={`font-bold ${isCompleted ? 'line-through text-gray-400' : ''}`}>{task.title}</p>
              <FaEdit className="cursor-pointer hover:text-gray-300" onClick={handleEditClick} />
            </div>
            <p className={isCompleted ? 'line-through text-gray-400' : ''}>{task.description}</p>
            <div className="flex space-x-2 mt-2">
              {task.status === Status.DONE ? (
                <div 
                  className="flex items-center space-x-1 px-3 py-1 border border-blue-400 rounded-full cursor-pointer text-blue-400 hover:bg-blue-400 hover:text-white transition-colors"
                  onClick={() => updateTaskStatus(task.id, task.status as Status, Status.TODO)}
                >
                  <ImArrowLeft2 className="text-xs" />
                  <span className="text-sm">Again</span>
                </div>
              ) : (
                <>
                  {task.status !== Status.DOING && (
                    <div 
                      className="flex items-center space-x-1 px-3 py-1 border border-green-400 rounded-full cursor-pointer text-green-400 hover:bg-green-400 hover:text-white transition-colors"
                      onClick={() => updateTaskStatus(task.id, task.status as Status, Status.DOING)}
                    >
                      <FaPlay className="text-xs" />
                      <span className="text-sm">Start</span>
                    </div>
                  )}
                  {task.status !== Status.TODO && (
                    <div 
                      className="flex items-center space-x-1 px-3 py-1 border border-blue-400 rounded-full cursor-pointer text-blue-400 hover:bg-blue-400 hover:text-white transition-colors"
                      onClick={() => updateTaskStatus(task.id, task.status as Status, Status.TODO)}
                    >
                      <FaPause className="text-xs" />
                      <span className="text-sm">Cancel</span>
                    </div>
                  )}
                  {(task.status as Status) !== Status.DONE && (
                    <div 
                      className="flex items-center space-x-1 px-3 py-1 border border-gray-400 rounded-full cursor-pointer text-gray-400 hover:bg-gray-400 hover:text-white transition-colors"
                      onClick={() => updateTaskStatus(task.id, task.status as Status, Status.DONE)}
                    >
                      <MdDone className="text-xs" />
                      <span className="text-sm">Complete</span>
                    </div>
                  )}
                </>
              )}
            </div>
          </>
        )}
      </div>
    </div>
  );
}

export default Item;
