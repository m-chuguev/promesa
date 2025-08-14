import { CommonModule, DatePipe } from '@angular/common';
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { TuiInputModule, TuiInputDateModule, TuiTagModule, TuiTextareaModule } from '@taiga-ui/kit';

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
    FormsModule,
    TuiInputModule,
    TuiInputDateModule,
    TuiTextareaModule,
    TuiTagModule,
    DatePipe,
  ],
  templateUrl: './notes-page.component.html',
  styleUrl: './notes-page.component.less',
})
export class NotesPageComponent {
  protected readonly tags: string[] = ['Работа', 'Личное', 'Идеи'];
  protected selectedTag = this.tags[0];
  protected titleFilter = '';
  protected dateFilter: Date | null = null;

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

  protected get filteredNotes(): Note[] {
    return this.notes.filter((note) => {
      const matchesTag = !this.selectedTag || note.tag === this.selectedTag;
      const matchesTitle =
        !this.titleFilter ||
        note.title.toLowerCase().includes(this.titleFilter.toLowerCase());
      const matchesDate =
        !this.dateFilter ||
        note.date.toDateString() === this.dateFilter.toDateString();
      return matchesTag && matchesTitle && matchesDate;
    });
  }

  protected selectTag(tag: string): void {
    this.selectedTag = tag;
    this.selectedNote = null;
  }

  protected selectNote(note: Note): void {
    this.selectedNote = note;
  }
}

