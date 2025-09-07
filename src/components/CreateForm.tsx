import { SubmitHandler, useForm } from "react-hook-form";
import { invoke } from "@tauri-apps/api/core";

type Inputs = {
  title: string,
  description: string
}

function CreateForm({setIsShowForm, reload}: {setIsShowForm: (isShowForm: boolean) => void, reload: () => void}) {

  const {
    register,
    handleSubmit,
    watch,
    formState: {errors},
    reset
  } = useForm<Inputs>();
  const onSubmit: SubmitHandler<Inputs> = (data) => {
    setIsShowForm(false);

    invoke('create_task', {title: data.title, description: data.description})
      .then(() => {
        reset();
        reload();
      })
      .catch(err => {
        console.error('create failed:', err);
      });
  }

  const resetValidationErrorStyle = () => { 
    const input = document.querySelector('#title');
    input?.classList.remove('border-red-500 focus:ring-red-500');
    input?.classList.add('border-gray-300 focus:ring-blue-500');
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="max-w-md mx-auto p-4">
      <p className="text-2xl">Add a new task</p>
      <div className="mb-4">
        <input 
          id="title"
          type="text" 
          defaultValue="" 
          placeholder="title"
          autoCapitalize="off"
          className={`w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:border-transparent ${errors.title ? 'border-red-500 focus:ring-red-500' : 'border-gray-300 focus:ring-blue-500'}`}
          {...register("title", { required: "Title is required" })} 
        />
        {errors.title && <p className="text-red-500 text-sm mt-1 text-left font-bold">{errors.title.message}</p>}
      </div>
      <div className="mb-4">
        <textarea 
          defaultValue="" 
          placeholder="description"
          rows={4}
          autoCapitalize="off"
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent resize-vertical"
          {...register("description")} 
        />
      </div>
      <input 
        type="submit" 
        value="Create Task"
        className="w-full bg-blue-500 text-white py-2 px-4 rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition duration-200"
      />
    </form>
  );
}

export default CreateForm;
