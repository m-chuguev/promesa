import { Route } from '@angular/router';

export const appRoutes: Route[] = [
  {
    path: 'todo/:id',
    loadComponent: () =>
      import('./todo/todo-edit.component').then((m) => m.TodoEditComponent),
  },
];
