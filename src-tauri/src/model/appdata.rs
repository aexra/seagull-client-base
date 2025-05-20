#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use crate::os;

#[derive(Clone, Serialize, Deserialize)]
pub struct AppData {
    pub api_url: String,
    pub access_token: String,
    pub refresh_token: String
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            api_url: String::from("http://localhost:8080/"),
            access_token: String::from("Aboba"),
            refresh_token: String::from("Bebebe"),
        }
    }
}

impl AppData {
    pub fn load_or_default() -> Self {
        match os::appdata::load_appdata() {
            Ok(data) => data,
            Err(_) => {
                let data = AppData::default();
                if let Err(e) = os::appdata::save_appdata(&data) {
                    panic!("Error while saving default AppData: {}", e);
                }
                data
            }
        }
    }
}
