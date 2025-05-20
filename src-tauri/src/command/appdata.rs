use std::sync::Mutex;
use tauri::State;

use crate::os;
use crate::model;

#[tauri::command]
pub fn get_appdata(state: State<'_, Mutex<model::appdata::AppData>>) -> model::appdata::AppData {
    let data = state.lock().unwrap();
    data.clone()
}

#[tauri::command]
pub fn set_appdata(
    state: State<'_, Mutex<model::appdata::AppData>>,
    new: model::appdata::AppData,
) -> Result<model::appdata::AppData, String> {
    if let Err(e) = os::appdata::save_appdata(&new) {
        return Err(format!("Failed to save app data: {}", e));
    }
    
    let mut data = state.lock().unwrap();
    *data = new.clone();

    Ok(data.clone())
}

