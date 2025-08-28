import { Task } from '../dto';
import { Status } from '../enum';
import Item from './Item';

function Lane({ tasks, statusLabel, updateTaskStatus }: { tasks: Task[]; statusLabel: Status; updateTaskStatus: (taskId: string, newStatus: Status) => void }) {
  const filteredTasks = tasks.filter(task => task.status === statusLabel);

  return (
    <div className='mr-3 border border-slate-400 rounded-lg mt-3'>
      <h2>{statusLabel}</h2>
      {filteredTasks.map(task => (
        <Item key={task.id} task={task} updateTaskStatus={updateTaskStatus} />
      ))}
    </div>
  );
}

export default Lane;
