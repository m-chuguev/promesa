// src-tauri/src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use tauri::{Manager, RunEvent, WebviewUrl};
use tauri::webview::WebviewWindowBuilder;
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_global_shortcut::{
    Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
};

// ----- macOS: раскладконезависимый Cmd+C через Quartz CGEvent -----
#[cfg(target_os = "macos")]
fn emulate_copy() {
    use core_graphics::event::{CGEvent, CGEventFlags, CGEventTapLocation};
    use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
    // виртуальный код ANSI для клавиши 'C'
    const ANSI_C: u16 = 8;

    let src = CGEventSource::new(CGEventSourceStateID::HIDSystemState).unwrap();

    // KeyDown 'C' с флагом Command
    let mut c_down = CGEvent::new_keyboard_event(src.clone(), ANSI_C as u16, true).unwrap();
    c_down.set_flags(CGEventFlags::CGEventFlagCommand);
    c_down.post(CGEventTapLocation::HID);

    // KeyUp 'C' с флагом Command
    let mut c_up = CGEvent::new_keyboard_event(src, ANSI_C as u16, false).unwrap();
    c_up.set_flags(CGEventFlags::CGEventFlagCommand);
    c_up.post(CGEventTapLocation::HID);
}

// ----- Win/Linux: Ctrl+C через Enigo 0.5 -----
#[cfg(not(target_os = "macos"))]
fn emulate_copy() {
    use enigo::{Direction, Enigo, Key, Keyboard, Settings};
    if let Ok(mut enigo) = Enigo::new(&Settings::default()) {
        let _ = enigo.key(Key::Control, Direction::Press);
        let _ = enigo.key(Key::Unicode('c'), Direction::Click);
        let _ = enigo.key(Key::Control, Direction::Release);
    }
}

fn new_label() -> String {
    let ms = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    format!("note-{ms}")
}

fn open_note_with_query(app: &tauri::AppHandle, text: String) {
    let route = if text.is_empty() {
        "todo/new".to_string()
    } else {
        format!("todo/new?text={}", urlencoding::encode(&text))
    };

    let label = new_label();
    if let Ok(win) =
        WebviewWindowBuilder::new(app, &label, WebviewUrl::App(route.into()))
            .title("Заметка")
            .build()
    {
        let _ = win.set_focus();
    }
}

fn main() {
    let app = tauri::Builder::default()
        .setup(|app| {
            // плагины
            app.handle().plugin(tauri_plugin_clipboard_manager::init());
            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_handler(|app, shortcut, event| {
                        // macOS: Cmd + Shift + Backslash
                        let mac = Shortcut::new(
                            Some(Modifiers::META | Modifiers::SHIFT),
                            Code::Backslash,
                        );
                        // (опционально) Win/Linux: Ctrl + Shift + Backslash
                        let win = Shortcut::new(
                            Some(Modifiers::CONTROL | Modifiers::SHIFT),
                            Code::Backslash,
                        );

                        // Срабатываем на отпускание (Released), чтобы модификаторы не мешали Copy
                        if event.state() == ShortcutState::Released && (*shortcut == mac || *shortcut == win) {
                            emulate_copy();
                            // ждём, пока ОС обновит буфер
                            thread::sleep(Duration::from_millis(120));
                            let text = app.clipboard().read_text().unwrap_or_default();
                            open_note_with_query(app, text);
                        }
                    })
                    .build(),
            );

            // регистрируем хоткеи
            let gs = app.global_shortcut();
            gs.register(Shortcut::new(
                Some(Modifiers::META | Modifiers::SHIFT),
                Code::Backslash,
            ))?; // macOS
            gs.register(Shortcut::new(
                Some(Modifiers::CONTROL | Modifiers::SHIFT),
                Code::Backslash,
            ))?; // Win/Linux (по желанию)

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("build failed");

    // держим процесс живым после закрытия последнего окна
    app.run(|_, event| {
        if let RunEvent::ExitRequested { api, .. } = event {
            api.prevent_exit();
        }
    });
}
