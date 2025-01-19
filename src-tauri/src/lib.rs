mod alist_command;
use alist_command::{AlistState, start_alist, stop_alist, get_alist_status};
mod alist_download;
use alist_download::download_and_extract_alist;
mod alist_version;
use alist_version::get_alist_version;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AlistState(std::sync::Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            greet,
            start_alist,
            stop_alist,
            get_alist_status,
            download_and_extract_alist,
            get_alist_version
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}