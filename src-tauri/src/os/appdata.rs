use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

use crate::model;

pub fn load_appdata() -> Result<model::appdata::AppData, Box<dyn std::error::Error>> {
    let proj_dirs = ProjectDirs::from("com", "Seagull", "Seagull")
        .ok_or("Failed to determine project directories")?;
    
    let mut file_path = PathBuf::from(proj_dirs.data_dir());
    file_path.push("appdata.json");

    if !file_path.exists() {
        return Err(format!("appdata.json does not exist in file path: {}", file_path.display()).into());
    }

    let json = fs::read_to_string(file_path)?;
    let data = serde_json::from_str(&json)?;
    
    Ok(data)
}

pub fn save_appdata(data: &model::appdata::AppData) -> Result<(), Box<dyn std::error::Error>> {
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
