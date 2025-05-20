use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;

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
    let mut data = state.lock().unwrap();
    *data = new;

    if let Err(e) = save_appdata(&data) {
        return Err(format!("Failed to save app data: {}", e));
    }

    Ok(data.clone())
}

fn save_appdata(data: &model::appdata::AppData) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "Seagull", "Seagull") {
        let data_dir = proj_dirs.data_dir();

        fs::create_dir_all(data_dir)?;

        let mut file_path = PathBuf::from(data_dir);
        file_path.push("appdata.json");

        let json = serde_json::to_string(data)?;
        fs::write(file_path, json)?;
    }

    Ok(())
}
