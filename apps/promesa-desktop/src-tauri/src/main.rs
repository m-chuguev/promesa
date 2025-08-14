#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{Manager, WebviewUrl, RunEvent};
use tauri::webview::WebviewWindowBuilder;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

fn new_note_id() -> String {
    let ms = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    format!("note-{ms}")
}

fn open_note_window(app: &tauri::AppHandle, id: &str) {
    let route = format!("todo/{id}");

    // создаём окно сразу на нужном Angular-маршруте
    let w = WebviewWindowBuilder::new(
        app,
        &format!("note-{}", id),                 // уникальный label на всякий случай
        WebviewUrl::App(route.into())
    )
        .title("Заметка")
        .build();

    if let Ok(win) = w {
        #[cfg(target_os = "macos")]
        app.set_activation_policy(tauri::ActivationPolicy::Regular);
        let _ = win.set_focus();
        // Никаких prevent_close: окно действительно закрывается по Cmd+W
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn main() {
    // 1) Собираем приложение (через build), чтобы иметь доступ к .run(|..., event| ...)
    let app = tauri::Builder::default()
        .setup(|app| {
            // macOS: пока нет окна — убираем иконку из Dock (необязательно)
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            // глобальный хоткей — Alt/Option + Shift + N
            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_handler(|app, shortcut, event| {
                        let combo = Shortcut::new(Some(Modifiers::ALT | Modifiers::SHIFT), Code::KeyN);
                        if event.state() == ShortcutState::Pressed && *shortcut == combo {
                            let id = new_note_id();
                            open_note_window(app, &id);
                        }
                    })
                    .build(),
            );

            app.global_shortcut()
                .register(Shortcut::new(Some(Modifiers::ALT | Modifiers::SHIFT), Code::KeyN))?;

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("build failed");

    // 2) НЕ ДАЁМ ПРОЦЕССУ ВЫЙТИ, когда закрыли последнее окно
    app.run(|_app_handle, event| {
        if let RunEvent::ExitRequested { api, .. } = event {
            api.prevent_exit(); // <-- ключевая строка
            // Процесс остаётся жить, глобальный хоткей активен.
        }
    });
}
