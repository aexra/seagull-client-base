use std::sync::Mutex;
use tauri::State;

use crate::model;

#[tauri::command]
pub fn get_appdata(state: State<'_, Mutex<model::appdata::AppData>>) -> model::appdata::AppData {
  let data = state.lock().unwrap();
  data.clone()
}

#[tauri::command]
pub fn set_appdata(state: State<'_, Mutex<model::appdata::AppData>>, new: model::appdata::AppData) -> model::appdata::AppData {
  let mut data = state.lock().unwrap();
  *data = new;
  data.clone()
}