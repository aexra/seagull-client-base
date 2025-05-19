pub struct AppData {
  pub welcome_message: String,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            welcome_message: String::from("Hello"),
        }
    }
}