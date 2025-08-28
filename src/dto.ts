export class Task {
  id: number;
  title: string;
  status: 'TODO' | 'DOING' | 'DONE' | 'PENDING';
  description: string;

  constructor(id: number, title: string, status: 'TODO' | 'DOING' | 'DONE' | 'PENDING' = 'TODO', description: string = '') {
    this.id = id;
    this.title = title;
    this.status = status;
    this.description = description;
  }
}

