import { Task } from '../dto';
import { Status } from '../enum';
import Item from './Item';

function Lane({ tasks, statusLabel, updateTaskStatus }: { tasks: Task[]; statusLabel: Status; updateTaskStatus: (taskId: number, newStatus: Status) => void }) {
  const filteredTasks = tasks.filter(task => task.status === statusLabel);

  return (
    <div>
      <h2>{statusLabel}</h2>
      {filteredTasks.map(task => (
        <Item key={task.id} task={task} updateTaskStatus={updateTaskStatus} />
      ))}
    </div>
  );
}

export default Lane;
