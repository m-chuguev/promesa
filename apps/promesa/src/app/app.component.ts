import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { TuiRoot } from '@taiga-ui/core';

@Component({
  imports: [RouterOutlet, TuiRoot],
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrl: './app.component.less',
})
export class AppComponent {
  title = 'promesa';
}
