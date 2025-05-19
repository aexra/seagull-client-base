use std::sync::Mutex;
use tauri::State;

use crate::model;

#[tauri::command]
pub fn get_api_url(state: State<'_, Mutex<model::appdata::AppData>>) -> String {
  let data = state.lock().unwrap();
  data.api_url.clone()
}