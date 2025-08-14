import {Injectable, signal} from "@angular/core";

@Injectable({
  providedIn: 'root'
})
export class TodoService {
  public counter = signal(0);
  public value = '';

}
