import { Task } from '../dto';
import { Status } from '../enum';
import Item from './Item';
import { FaRegTrashAlt } from "react-icons/fa";

function Lane({ tasks, statusLabel, updateTaskStatus, reload, fadingTasks, fadingInTasks, onDeleteDoneTasks }: { tasks: Task[]; statusLabel: Status; updateTaskStatus: (taskId: string, oldStatus: Status, newStatus: Status) => void; reload: () => void; fadingTasks: Set<string>; fadingInTasks: Set<string>; onDeleteDoneTasks?: () => void }) {
  const filteredTasks = tasks.filter(task => task.status === statusLabel).sort((a, b) => a.id.localeCompare(b.id));

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
    <div className={`mx-2 border rounded-lg mt-3 flex-1 min-w-48 max-w-80 transition-all duration-500 ease-in-out ${getStatusColor()}`}>
      <div className="text-white font-bold p-2 text-center relative">
        <h2 className="inline-block">{statusLabel.toUpperCase()}</h2>
        {statusLabel === Status.DONE && onDeleteDoneTasks && (
          <button
            onClick={onDeleteDoneTasks}
            className="absolute right-2 top-1/2 -translate-y-1/2 bg-gray-600 hover:bg-gray-700 cursor-pointer text-white text-sm px-2 py-1 rounded transition-colors"
          >
            <FaRegTrashAlt />
          </button>
        )}
      </div>
      <div className="transition-all duration-500 ease-in-out">
        {filteredTasks.map(task => {
          const isFading = fadingTasks.has(task.id);
          const isFadingIn = fadingInTasks.has(task.id);
          
          let animationClass = '';
          if (isFading) {
            animationClass = 'opacity-0 scale-95 translate-y-2';
          } else if (isFadingIn) {
            animationClass = 'opacity-100 scale-100 translate-y-0 animate-[fadeInUp_0.3s_ease-out]';
          } else {
            animationClass = 'opacity-100 scale-100 translate-y-0';
          }
          
          return (
            <div 
              key={task.id} 
              className={`transition-all duration-300 ease-in-out transform ${animationClass}`}
            >
              <Item task={task} updateTaskStatus={updateTaskStatus} reload={reload} />
            </div>
          );
        })}
      </div>
    </div>
  );
}

export default Lane;
