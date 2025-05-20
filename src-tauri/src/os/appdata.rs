use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

use crate::model;

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
