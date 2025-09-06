import { FaPause, FaPlay } from "react-icons/fa";
import { Task } from "../dto";
import { Status } from "../enum";
import { LuUndo2 } from "react-icons/lu";
import { MdDone } from "react-icons/md";

function Item({ task, updateTaskStatus }: { task: Task; updateTaskStatus: (taskId: string, oldStatus: Status, newStatus: Status) => void }) {
  return (
    <div className="border border-gray-300 p-2 m-1">
      <p className="font-bold">{task.title}</p>
      <p>{task.description}</p>
      <div className="flex">
        <div onClick={() => updateTaskStatus(task.id, task.status as Status, Status.DOING)}>
          <FaPlay />
        </div>
        <div onClick={() => updateTaskStatus(task.id, task.status as Status, Status.PENDING)}>
          <FaPause />
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
