import { FaPause, FaPlay, FaEdit } from "react-icons/fa";
import { Task } from "../dto";
import { Status } from "../enum";
import { LuUndo2 } from "react-icons/lu";
import { MdDone } from "react-icons/md";
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
  return (
    <div className="border border-gray-300 p-2 m-1">
      <div className="text-left">
        {isEditing ? (
          <form onSubmit={handleSubmit(onSubmit)} className="space-y-2">
            <div>
              <input
                type="text"
                autoCapitalize="off"
                className={`w-full px-2 py-1 border rounded text-sm ${errors.title ? 'border-red-500' : 'border-gray-300'}`}
                {...register("title", { required: "Title is required" })}
              />
              {errors.title && <p className="text-red-500 text-xs">{errors.title.message}</p>}
            </div>
            <div>
              <textarea
                rows={3}
                autoCapitalize="off"
                className="w-full px-2 py-1 border border-gray-300 rounded text-sm resize-vertical"
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
              <p className="font-bold">{task.title}</p>
              <FaEdit className="cursor-pointer" onClick={handleEditClick} />
            </div>
            <p>{task.description}</p>
          </>
        )}
      </div>
      <div className="flex">
        <div onClick={() => updateTaskStatus(task.id, task.status as Status, Status.DOING)}>
          <FaPlay className="cursor-pointer" />
        </div>
        <div onClick={() => updateTaskStatus(task.id, task.status as Status, Status.PENDING)}>
          <FaPause className="cursor-pointer" />
        </div>
        <div onClick={() => updateTaskStatus(task.id, task.status as Status, Status.TODO)}>
          <LuUndo2 />
        </div>
        <div onClick={() => updateTaskStatus(task.id, task.status as Status, Status.DONE)}>
          <MdDone />
        </div>
      </div>
    </div>
  );
}

export default Item;
