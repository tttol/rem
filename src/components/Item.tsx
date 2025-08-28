import { Task } from "../dto";
import { Status } from "../enum";

function Item({ task, updateTaskStatus }: { task: Task; updateTaskStatus: (taskId: number, newStatus: Status) => void }) {
  const availableStatuses = Object.values(Status).filter(status => status !== task.status);

  return (
    <div style={{ border: '1px solid #ccc', padding: '8px', margin: '4px' }}>
      <h3>{task.title}</h3>
      <p>{task.description}</p>
      <p>Current Status: {task.status}</p>
      <div>
        {availableStatuses.map(status => (
          <button
            key={status}
            onClick={() => updateTaskStatus(task.id, status)}
            style={{ marginRight: '4px' }}
          >
            Move to {status}
          </button>
        ))}
      </div>
    </div>
  );
}

export default Item;
