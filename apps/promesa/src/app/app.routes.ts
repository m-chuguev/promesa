import { Route } from '@angular/router';
import { NotesPageComponent } from './notes-page/notes-page.component';

export const appRoutes: Route[] = [
  {
    path: '',
    component: NotesPageComponent,
  },
];
