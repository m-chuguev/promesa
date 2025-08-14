import { CommonModule } from '@angular/common';
import {Component, inject, OnDestroy} from '@angular/core';
import { FormBuilder, ReactiveFormsModule } from '@angular/forms';
import { ActivatedRoute } from '@angular/router';
import {TodoService} from "./todo.service";

@Component({
  selector: 'app-todo-edit',
  standalone: true,
  imports: [CommonModule, ReactiveFormsModule],
  template: `
    <h2>Edit note {{ id }}</h2>
    <h4>destroy: {{ todoService.counter() }} {{todoService.value}}</h4>
    <form [formGroup]="form" (ngSubmit)="save()">
      <label for="note">Note</label>
      <input id="note" type="text" formControlName="note" />
      <button type="submit">Save</button>
    </form>
  `,
})
export class TodoEditComponent implements OnDestroy {
  private readonly fb = inject(FormBuilder);
  private readonly route = inject(ActivatedRoute);
  public todoService = inject(TodoService);

  readonly id = this.route.snapshot.paramMap.get('id');
  readonly form = this.fb.group({
    note: [''],
  });

  constructor() {

    this.form.valueChanges.subscribe(value => {
      this.todoService.value = value.note ?? ''
    })
  }

  save() {
    if (this.form.valid) {
      console.log(`Saving note ${this.id}:`, this.form.value.note);
    }
  }

  ngOnDestroy() {
    this.todoService.counter.set(this.todoService.counter() + 1);
  }
}
