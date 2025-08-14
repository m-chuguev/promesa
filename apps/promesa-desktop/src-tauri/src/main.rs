#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tauri::{Manager, WebviewUrl, RunEvent, Emitter};
use tauri::webview::WebviewWindowBuilder;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

use tauri_plugin_clipboard_manager::ClipboardExt;


// Имитируем системное Copy, чтобы в буфер попал выделенный текст
fn press_system_copy() -> anyhow::Result<()> {
    use enigo::{Enigo, Key, Keyboard, Direction, Settings};
    let mut enigo = Enigo::new(&Settings::default())?;

    #[cfg(target_os = "macos")]
    {
        enigo.key(Key::Meta, Direction::Press)?;              // Cmd down
        enigo.key(Key::Unicode('c'), Direction::Click)?;       // 'c'
        enigo.key(Key::Meta, Direction::Release)?;            // Cmd up
    }

    #[cfg(not(target_os = "macos"))]
    {
        enigo.key(Key::Control, Direction::Press)?;           // Ctrl down
        enigo.key(Key::Unicode('c'), Direction::Click)?;       // 'c'
        enigo.key(Key::Control, Direction::Release)?;         // Ctrl up
    }

    Ok(())
}

fn new_note_label() -> String {
    let ms = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    format!("note-{ms}")
}

fn open_note_with_clipboard_text(app: &tauri::AppHandle) {
    // 1) Копируем выделение
    press_system_copy();
    thread::sleep(Duration::from_millis(120)); // маленькая пауза, чтобы буфер успел обновиться

    // 2) Читаем буфер
    let text = app.clipboard().read_text().unwrap_or_default();
    let q = urlencoding::encode(&text);

    // 3) Открываем Angular на todo/new с автозаполнением через query
    let label = new_note_label();
    let route = format!("todo/new?text={q}");
    let w = WebviewWindowBuilder::new(app, &label, WebviewUrl::App(route.into()))
        .title("Заметка")
        .build();

    if let Ok(win) = w {
        let _ = win.set_focus();
        let _ = app.emit_to(&label, "note-text", q);
    }
    //
    // let app_handle = app.clone();
    //
    // thread::spawn(move || {
    //     thread::sleep(Duration::from_millis(250));    // ← задержка
    //     if let Some(wnd) = app_handle.get_webview_window(&label) {
    //         let _ = wnd.emit("note-text", serde_json::json!({ "text": q }));
    //     }
    // });
    //
    // {
    //     #[cfg(target_os = "macos")]
    //     app.set_activation_policy(tauri::ActivationPolicy::Regular);
    //     // Никаких prevent_close — окно реально закроется по Cmd+W
    // }
    // if let Ok(win) = WebviewWindowBuilder::new(app, &label, WebviewUrl::App(route.into()))
    //     .title("Заметка")
    //     .build()
    // {
    //     let _ = win.set_focus();
    //     // std::thread::sleep(std::time::Duration::from_millis(1500));
    //     let _ = app.emit_to(&label, "note-text", serde_json::json!({ "text": q }));
    //
    //     // // данные, которые понадобятся в другом потоке
    //     // let app_handle = app.clone();
    //     // let label_for_thread = label.clone();
    //     // let payload = serde_json::json!({ "text": q });
    //     //
    //     // // отдельный поток: ждём 250 мс и только потом шлём событие в это окно
    //     // thread::spawn(move || {
    //     //     thread::sleep(Duration::from_millis(250));                  // чтобы фронт успел подписаться
    //     //     let _ = app_handle.emit_to(&label_for_thread, "note-text", payload);   // <— ВАЖНО: emit_to по label
    //     // });
    // }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn main() {
    // Соберём, чтобы иметь доступ к app.run(|_, event|)
    let app = tauri::Builder::default()
        .setup(|app| {
            // macOS: скрываем иконку из Dock, пока окна нет
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            // Плагины
            app.handle().plugin(tauri_plugin_clipboard_manager::init());
            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_handler(|app, shortcut, event| {
                        let combo = Shortcut::new(Some(Modifiers::ALT | Modifiers::SHIFT), Code::KeyN);
                        if event.state() == ShortcutState::Pressed && *shortcut == combo {
                            open_note_with_clipboard_text(app);
                        }
                    })
                    .build(),
            );

            // Регистрируем хоткей Alt/Option + Shift + N
            app.global_shortcut()
                .register(Shortcut::new(Some(Modifiers::ALT | Modifiers::SHIFT), Code::KeyN))?;

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("build failed");

    // Не даём процессу выйти, когда закрыто последнее окно — хоткей остаётся активным
    app.run(|_handle, event| {
        if let RunEvent::ExitRequested { api, .. } = event {
            api.prevent_exit();
        }
    });
}