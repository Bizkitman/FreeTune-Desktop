#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod search;
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![search::search_tracks])
        .run(tauri::generate_context!())
        .expect("error while running FreeTune");
}
