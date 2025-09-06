import { IoIosAddCircleOutline } from 'react-icons/io';
import { Task } from '../dto';
import { Status } from '../enum';
import Item from './Item';

function Lane({ tasks, statusLabel, updateTaskStatus }: { tasks: Task[]; statusLabel: Status; updateTaskStatus: (taskId: string, oldStatus: Status, newStatus: Status) => void }) {
  const filteredTasks = tasks.filter(task => task.status === statusLabel);

  return (
    <div className='mx-2 border border-slate-400 rounded-lg mt-3 flex-1 min-w-48 max-w-80'>
      <h2>{statusLabel}</h2>
      {filteredTasks.map(task => (
        <Item key={task.id} task={task} updateTaskStatus={updateTaskStatus} />
      ))}
      {statusLabel == Status.TODO && 
        <div className="border border-gray-300 p-2 m-1">
          <div className='flex items-center'>
            <IoIosAddCircleOutline />
            <div>Add new task</div>
          </div>
        </div>
      }
    </div>
  );
}

export default Lane;
