// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use actix_web::rt::System;
use std::thread;

use semtex_api::run_server;
use tauri::Manager;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let show_hide = CustomMenuItem::new("show_hide".to_string(), "Show/Hide");

    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(show_hide);
    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "show_hide" => match app.get_window("main") {
                    Some(window) => match window.is_visible().unwrap() {
                        true => window.hide().unwrap(),
                        false => {
                            window.show().unwrap();
                            window.set_focus().unwrap()
                        }
                    },
                    _ => ()
                },
                _ => {}
            },
            _ => {}
        })
        .setup(|app| {
            let handle = app.handle();
            let boxed_handle = Box::new(handle);

            thread::spawn(move || {
                System::new().block_on(run_server()).unwrap();
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
