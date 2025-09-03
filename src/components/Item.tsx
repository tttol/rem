import { FaPause, FaPlay } from "react-icons/fa";
import { Task } from "../dto";
import { Status } from "../enum";
import { LuUndo2 } from "react-icons/lu";
import { MdDone } from "react-icons/md";

function Item({ task, updateTaskStatus }: { task: Task; updateTaskStatus: (taskId: string, newStatus: Status) => void }) {
  // const availableStatuses = Object.values(Status).filter(status => status !== task.status);

  return (
    <div style={{ border: '1px solid #ccc', padding: '8px', margin: '4px' }}>
      <h3>{task.title}</h3>
      <p>{task.description}</p>
      <p>Current Status: {task.status}</p>
      <div>
        <div onClick={() => updateTaskStatus(task.id, Status.DOING)}>
          <FaPlay />
        </div>
        <div onClick={() => updateTaskStatus(task.id, Status.PENDING)}>
          <FaPause />
        </div>
        <div onClick={() => updateTaskStatus(task.id, Status.TODO)}>
          <LuUndo2 />
        </div>
        <div onClick={() => updateTaskStatus(task.id, Status.DONE)}>
          <MdDone />
        </div>
      </div>
    </div>
  );
}

export default Item;
