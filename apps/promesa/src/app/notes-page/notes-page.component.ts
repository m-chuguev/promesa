import { CommonModule, DatePipe } from '@angular/common';
import { Component } from '@angular/core';
import { FormControl, FormGroup, ReactiveFormsModule } from '@angular/forms';
import { TuiInputDate, TuiTag, TuiTextarea } from '@taiga-ui/kit';
import { TuiTextfieldControllerModule, TuiTextfield } from '@taiga-ui/core';
import { TuiDay } from '@taiga-ui/cdk';
import { CdkDragDrop, DragDropModule, moveItemInArray } from '@angular/cdk/drag-drop';
import { takeUntilDestroyed } from '@angular/core/rxjs-interop';

interface Note {
  id: number;
  title: string;
  content: string;
  tag: string;
  date: Date;
}

@Component({
  selector: 'app-notes-page',
  imports: [
    CommonModule,
    ReactiveFormsModule,
    TuiInputDate,
    TuiTag,
    TuiTextarea,
    TuiTextfield,
    TuiTextfieldControllerModule,
    DragDropModule,
    DatePipe,
  ],
  templateUrl: './notes-page.component.html',
  styleUrl: './notes-page.component.less',
})
export class NotesPageComponent {
  protected readonly tags: string[] = ['Работа', 'Личное', 'Идеи'];
  protected selectedTag = this.tags[0];

  protected readonly filtersForm = new FormGroup({
    title: new FormControl('', { nonNullable: true }),
    date: new FormControl<TuiDay | null>(null),
  });

  protected readonly noteForm = new FormGroup({
    title: new FormControl('', { nonNullable: true }),
    content: new FormControl('', { nonNullable: true }),
  });

  protected notes: Note[] = [
    {
      id: 1,
      title: 'Первая заметка',
      content: 'Содержимое первой заметки',
      tag: 'Работа',
      date: new Date(2024, 1, 10),
    },
    {
      id: 2,
      title: 'Список покупок',
      content: 'Молоко, хлеб, масло',
      tag: 'Личное',
      date: new Date(2024, 2, 5),
    },
    {
      id: 3,
      title: 'Новая идея',
      content: 'Нужно записать новую идею',
      tag: 'Идеи',
      date: new Date(2024, 5, 15),
    },
  ];

  protected selectedNote: Note | null = this.notes[0];

  constructor() {
    if (this.selectedNote) {
      this.noteForm.setValue({
        title: this.selectedNote.title,
        content: this.selectedNote.content,
      });
    }

    this.noteForm.valueChanges.pipe(takeUntilDestroyed()).subscribe((value) => {
      if (this.selectedNote) {
        this.selectedNote.title = value.title;
        this.selectedNote.content = value.content;
      }
    });
  }

  protected get filteredNotes(): Note[] {
    const { title, date } = this.filtersForm.value as {
      title: string;
      date: TuiDay | null;
    };

    return this.notes.filter((note) => {
      const matchesTag = !this.selectedTag || note.tag === this.selectedTag;
      const matchesTitle =
        !title || note.title.toLowerCase().includes(title.toLowerCase());
      const matchesDate =
        !date ||
        note.date.toDateString() === date.toLocalNativeDate().toDateString();
      return matchesTag && matchesTitle && matchesDate;
    });
  }

  protected selectTag(tag: string): void {
    this.selectedTag = tag;
    this.selectedNote = null;
  }

  protected selectNote(note: Note): void {
    this.selectedNote = note;
    this.noteForm.setValue({ title: note.title, content: note.content });
  }

  protected drop(event: CdkDragDrop<string[]>): void {
    moveItemInArray(this.tags, event.previousIndex, event.currentIndex);
  }
}

