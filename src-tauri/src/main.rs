// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
    SystemTrayMenuItemHandle,
};
use tauri::Manager;

mod unread;

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);

            app.emit_all("single-instance", Payload { args: argv, cwd })
                .unwrap();
        }))
        .plugin(tauri_plugin_persisted_scope::init())
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                let item_handle: SystemTrayMenuItemHandle = app.tray_handle().get_item(&id);
                match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "hide" => {
                        let window = app.get_window("main").unwrap();
                        let visible = window.is_visible().expect("Cannot find window, somehow");
                        if visible {
                            window.hide().unwrap();
                            item_handle.set_title("Show").unwrap();
                        } else {
                            window.show().unwrap();
                            item_handle.set_title("Hide").unwrap();
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })

        .setup(|app| {
            let window = app.get_window("main").unwrap();
            tauri::async_runtime::spawn(async move {
                println!("Initializing...");

                let _loader = window.eval("window.location.replace('https://mail.google.com/chat/u/0')");

                let _hide = window.hide();
                // let _loader = window.eval("window.location.replace('https://mail.google.com/chat/u/0')");

                window
                    .get_window("main")
                    .expect("no window labeled 'main' found")
                    .show()
                    .unwrap();
            });

            // Poll the window title for the unread count and reflect it as a
            // numbered badge on the tray icon.
            let poll_window = app.get_window("main").unwrap();
            let tray = app.tray_handle();
            std::thread::spawn(move || {
                let mut last: i64 = -1;
                loop {
                    let count = poll_window
                        .title()
                        .map(|t| unread::parse_unread(&t))
                        .unwrap_or(0);
                    if count as i64 != last {
                        last = count as i64;
                        if let Some(icon) = unread::render_icon(count) {
                            let _ = tray.set_icon(icon);
                        }
                    }
                    std::thread::sleep(std::time::Duration::from_secs(2));
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error running tauri app");
}
