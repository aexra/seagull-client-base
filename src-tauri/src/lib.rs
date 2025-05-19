mod appdata;

use std::sync::Mutex;
use tauri::{State, Manager};

#[tauri::command]
fn greet(state: State<'_, Mutex<appdata::AppData>>, name: &str) -> String {
    // format!("Hello, {}! You've been greeted from Rust!", name)
    let mut state = state.lock().unwrap();
    state.welcome_message.push('1');
    format!("{}, {}", state.welcome_message, name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(appdata::AppData::default()));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
