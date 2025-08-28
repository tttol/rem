import { Status } from "./enum";

export class Task {
  id: string;
  title: string;
  // status: 'TODO' | 'DOING' | 'DONE' | 'PENDING';
  status: Status;
  description: string;

  constructor(id: string, title: string, status: Status, description: string = '') {
    this.id = id;
    this.title = title;
    this.status = status;
    this.description = description;
  }
}

