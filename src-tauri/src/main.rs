// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::Manager;
use window_shadows::set_shadow;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            // main_window.set_title("My Tauri App54545").unwrap();
            main_window.show().unwrap();
            set_shadow(&main_window, false).unwrap(); // 解决鼠标移动后，窗口阴影出现的问题
                                                      // This is where you can perform any setup tasks before the app starts
                                                      // For example, you can initialize a database connection or load configuration files
            println!("App is starting up...");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
