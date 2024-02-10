// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils;
mod station;
mod photo;
mod handle;

use std::sync::Mutex;
use crate::station::kml::{kml_to_excel, kml_to_json};
use crate::station::{STATION, Station};

fn main() {
    STATION.set(Mutex::new(Station::default())).unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            kml_to_excel,
            kml_to_json
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
