// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils;
mod station;
mod photo;
mod handle;

use station::kml::{kml_to_excel, kml_to_json};
use station::excel::excel_to_json;
use handle::{
    calc_photo,move_to_output
};

#[tokio::main]
async fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            kml_to_excel,
            kml_to_json,
            excel_to_json,
            calc_photo,
            move_to_output,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
