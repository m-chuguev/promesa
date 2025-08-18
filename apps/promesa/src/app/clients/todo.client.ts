import {Injectable} from "@angular/core";
import {from, Observable} from "rxjs";
import { fetch } from '@tauri-apps/plugin-http';

@Injectable({providedIn: 'root'})
export class TodoClient {
  public create(todo: any): Observable<any> {
    const res = fetch('https://webhook.site/8e21f5a5-6b1f-43fc-91b5-41f8eaa6d8ad', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(todo),
    });


    return from(res);
  }
}
