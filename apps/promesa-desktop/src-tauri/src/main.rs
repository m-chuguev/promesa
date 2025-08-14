#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{Manager, WebviewUrl};
use tauri::webview::WebviewWindowBuilder;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn main() {
  tauri::Builder::default()
    .setup(|app| {
      // подключаем плагин глобальных хоткеев
      app.handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
          .with_handler(|app, shortcut, event| {
            // реагируем на Option(Alt)+Shift+N
            let wanted = Shortcut::new(Some(Modifiers::ALT | Modifiers::SHIFT), Code::KeyN);
            if event.state() == ShortcutState::Pressed && *shortcut == wanted {
              // генерируем простой id (например, по времени)
              let ms = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
              let id = format!("note-{ms}");

              // путь Angular: todo/:id
              let route = format!("todo/{id}");

              // создаём окно сразу на нужном маршруте вашего Angular
              // В dev это будет <devUrl>/todo/:id, в проде — tauri://localhost/todo/:id
              let _ = WebviewWindowBuilder::new(app, &id, WebviewUrl::App(route.into()))
                .title("Новая заметка")
                .build()
                .and_then(|w| { w.set_focus().ok(); Ok(()) });
            }
          })
          .build(),
      );

      // регистрируем сам хоткей (Option/Alt + Shift + N)
      let gs = app.global_shortcut();
      gs.register(Shortcut::new(Some(Modifiers::ALT | Modifiers::SHIFT), Code::KeyN))?;

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
