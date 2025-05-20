use std::sync::Mutex;
use tauri::{Manager};

mod model {
    pub mod appdata;
}
mod command {
    pub mod appdata;
}
mod os {
    pub mod appdata;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(model::appdata::AppData::load_or_default()));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            command::appdata::get_appdata,
            command::appdata::set_appdata,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
