import { IoIosAddCircleOutline } from 'react-icons/io';
import { Task } from '../dto';
import { Status } from '../enum';
import Item from './Item';

function Lane({ tasks, statusLabel, updateTaskStatus, reload }: { tasks: Task[]; statusLabel: Status; updateTaskStatus: (taskId: string, oldStatus: Status, newStatus: Status) => void; reload: () => void }) {
  const filteredTasks = tasks.filter(task => task.status === statusLabel);

  const getStatusColor = () => {
    switch (statusLabel) {
      case Status.TODO:
        return 'bg-blue-900 border-blue-700';
      case Status.DOING:
        return 'bg-green-900 border-green-700';
      case Status.DONE:
        return 'bg-gray-800 border-gray-700';
      default:
        return 'bg-gray-700 border-gray-600';
    }
  };

  return (
    <div className={`mx-2 border rounded-lg mt-3 flex-1 min-w-48 max-w-80 ${getStatusColor()}`}>
      <h2 className="text-white font-bold p-2 text-center">{statusLabel.toUpperCase()}</h2>
      {filteredTasks.map(task => (
        <Item key={task.id} task={task} updateTaskStatus={updateTaskStatus} reload={reload} />
      ))}
    </div>
  );
}

export default Lane;
