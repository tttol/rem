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
    formState: {errors}
  } = useForm<Inputs>();
  const onSubmit: SubmitHandler<Inputs> = (data) => {
    setIsShowForm(false);

    invoke('create_task', {title: data.title, description: data.description})
      .then(() => {
        console.log('create success');
        reload();
      })
      .catch(err => {
        console.error('create failed:', err);
      });
  }

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="max-w-md mx-auto p-4">
      <p className="text-2xl">Add a new task</p>
      <div className="mb-4">
        <input 
          type="text" 
          defaultValue="" 
          placeholder="title"
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          {...register("title")} 
        />
      </div>
      <div className="mb-4">
        <textarea 
          defaultValue="" 
          placeholder="description"
          rows={4}
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
