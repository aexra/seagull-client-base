#![allow(dead_code)]
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
