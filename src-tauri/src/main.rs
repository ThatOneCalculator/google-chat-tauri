// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
    SystemTrayMenuItemHandle,
};
use tauri::{Manager, Window};

#[tauri::command]
async fn close_splashscreen(window: Window) {
    window
        .get_window("splashscreen")
        .expect("no window labeled 'splashscreen' found")
        .close()
        .unwrap();
    window
        .get_window("main")
        .expect("no window labeled 'main' found")
        .show()
        .unwrap();
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);

    tauri::Builder::default()
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
        .invoke_handler(tauri::generate_handler![close_splashscreen])
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            tauri::async_runtime::spawn(async move {
                println!("Initializing...");

                let check_page_js = "
                const invoke = window.__TAURI__.invoke

                const closeSplash = () => {
                    const currentLocation =
                    if (window.location.href.toString().includes(#onboarding)) {
                        invoke('close_splashscreen')
                        return true;
                    }
                    return false;
                }";

                let _loader =
                    window.eval("window.location.replace('https://mail.google.com/chat/u/0')");
                let _hide = window.hide();

                std::thread::sleep(std::time::Duration::from_millis(800));
                let _invoke_first = window.eval(check_page_js);

                std::thread::sleep(std::time::Duration::from_millis(2000));
                let _invoke_second = window.eval(check_page_js);

                std::thread::sleep(std::time::Duration::from_millis(2000));
                let _invoke_third = window.eval(check_page_js);

                std::thread::sleep(std::time::Duration::from_millis(2000));
                window
                    .get_window("splashscreen")
                    .expect("no window labeled 'splashscreen' found")
                    .close()
                    .unwrap();
                window
                    .get_window("main")
                    .expect("no window labeled 'main' found")
                    .show()
                    .unwrap();
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error running tauri app");
}
