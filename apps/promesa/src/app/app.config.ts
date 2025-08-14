import {ApplicationConfig, Injector, provideZoneChangeDetection} from '@angular/core';
import { provideRouter } from '@angular/router';
import { appRoutes } from './app.routes';
import {TUI_EDITOR_DEFAULT_EXTENSIONS, TUI_EDITOR_EXTENSIONS} from "@taiga-ui/editor";

export const appConfig: ApplicationConfig = {
  providers: [
    provideZoneChangeDetection({ eventCoalescing: true }),
    provideRouter(appRoutes),
    {
      provide: TUI_EDITOR_EXTENSIONS,
      deps: [Injector],
      useFactory: (injector: Injector) => [
        ...TUI_EDITOR_DEFAULT_EXTENSIONS,
        import('@taiga-ui/editor').then(({setup}) => setup({injector})),
      ],
    }
  ],
};
