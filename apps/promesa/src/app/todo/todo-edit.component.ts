import { CommonModule } from '@angular/common';
import {Component, DestroyRef, inject, OnDestroy} from '@angular/core';
import {FormBuilder, FormControl, ReactiveFormsModule} from '@angular/forms';
import { ActivatedRoute } from '@angular/router';
import {TodoService} from "./todo.service";
import {listen, once} from '@tauri-apps/api/event';
import {takeUntilDestroyed} from "@angular/core/rxjs-interop";
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import {TuiEditor} from "@taiga-ui/editor";


@Component({
  selector: 'app-todo-edit',
  standalone: true,
  imports: [CommonModule, ReactiveFormsModule, TuiEditor],
  template: `
    <h2>Edit note {{ id }}</h2>
    <h4>destroy: {{ todoService.counter() }} {{todoService.value}}</h4>
    <form [formGroup]="form" (ngSubmit)="save()">
      <label for="note">Note</label>
      <tui-editor
        formControlName="note"
      >
        Placeholder
      </tui-editor>
      <button type="submit">Save</button>
      {{payload | json}}
    </form>
  `,
})
export class TodoEditComponent implements OnDestroy {
  private readonly fb = inject(FormBuilder);
  private readonly route = inject(ActivatedRoute);
  private destroyRef = inject(DestroyRef);
  public todoService = inject(TodoService);
  private unlisten: any;
  public payload: any = {};

  readonly id = this.route.snapshot.paramMap.get('id');
  readonly form = this.fb.group({
    note: new FormControl(''),
  });

  // constructor() {
  //   const unlistenPromise = listen<{text: string}>('note-text', ({ payload }) => {
  //     this.form.controls.note.setValue(payload.text);
  //     this.todoService.value = payload.text;
  //   });
  //
  //
  //   this.form.valueChanges.subscribe(value => {
  //     this.todoService.value = value.note ?? ''
  //   })
  //
  //   // this.route.queryParamMap
  //   //   .pipe(takeUntilDestroyed(this.destroyRef))
  //   //   .subscribe(q => {
  //   //     const text = q.get('text') ?? '';
  //   //     this.form.controls.note.setValue(text);
  //   //   });
  // }

  ngOnInit() {
    // const appWebview = getCurrentWebviewWindow();
    // appWebview.once('note-text', (value) => {
    //   this.form.setValue({note: 'not value'});
    //   this.todoService.value = 'not value';
    //   this.payload = value;
    //
    // });
    //
    // this.unlisten = listen<{ text: string }>('note-text', (value) => {
    //   // console.log('[listen] got payload:', payload);
    //   // this.form.controls.note.setValue(payload?.text ?? 'not value');
    //   // this.todoService.value = payload?.text ?? 'not value';
    //   this.payload = value;
    // });

      this.route.queryParamMap
        .pipe(takeUntilDestroyed(this.destroyRef))
        .subscribe(q => {
          const text = q.get('text') ?? '';
          this.form.controls.note.setValue(text ?? 'not value');
        });
  }

  save() {
    if (this.form.valid) {
      console.log(`Saving note ${this.id}:`, this.form.value.note);
    }
  }

  ngOnDestroy() {
    this.todoService.counter.set(this.todoService.counter() + 1);
    this.unlisten?.();
  }
}
