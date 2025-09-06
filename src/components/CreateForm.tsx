import { SubmitHandler, useForm } from "react-hook-form";

type Inputs = {
  title: string,
  description: string
}

function CreateForm() {

  const {
    register,
    handleSubmit,
    watch,
    formState: {errors}
  } = useForm<Inputs>();
  const onSubmit: SubmitHandler<Inputs> = (data) => console.log(data);

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="max-w-md mx-auto p-4">
      <p>Add a new task</p>
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
