import { CommonModule } from '@angular/common';
import { Component, inject } from '@angular/core';
import { FormBuilder, ReactiveFormsModule } from '@angular/forms';
import { ActivatedRoute } from '@angular/router';

@Component({
  selector: 'app-todo-edit',
  standalone: true,
  imports: [CommonModule, ReactiveFormsModule],
  template: `
    <h2>Edit note {{ id }}</h2>
    <form [formGroup]="form" (ngSubmit)="save()">
      <label for="note">Note</label>
      <input id="note" type="text" formControlName="note" />
      <button type="submit">Save</button>
    </form>
  `,
})
export class TodoEditComponent {
  private readonly fb = inject(FormBuilder);
  private readonly route = inject(ActivatedRoute);

  readonly id = this.route.snapshot.paramMap.get('id');
  readonly form = this.fb.group({
    note: [''],
  });

  save() {
    if (this.form.valid) {
      console.log(`Saving note ${this.id}:`, this.form.value.note);
    }
  }
}
