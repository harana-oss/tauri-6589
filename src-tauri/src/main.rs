// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_global_shortcut::*;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {

          let window = app.get_window("main").unwrap();
          window.set_visible_on_all_workspaces(true)?;

          let hot_key: Shortcut = "command+shift+space".parse()?;

          app.handle().plugin(tauri_plugin_global_shortcut::Builder::with_handler(move |shortcut| {
              if shortcut == &hot_key {
                if window.is_visible().unwrap() {
                  window.hide().unwrap();
                } else {
                  window.show().unwrap();
                }
              } 

          }).build())?;

          app.handle().global_shortcut().register(hot_key)?;

          Ok(())
        })
        
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
