mod alist_command;
use alist_command::{AlistState, start_alist, stop_alist, get_alist_status, delete_data_folder};
mod alist_download;
use alist_download::download_and_extract_alist;
mod alist_version;
use alist_version::get_alist_version;
mod alist_password;
use alist_password::set_alist_password;

// 定义全局状态结构体
pub struct AlistPath(pub std::sync::Mutex<Option<String>>);

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AlistState(std::sync::Mutex::new(None)))
        .manage(AlistPath(std::sync::Mutex::new(None))) // 管理 AlistPath 状态
        .invoke_handler(tauri::generate_handler![
            greet,
            start_alist,
            stop_alist,
            get_alist_status,
            delete_data_folder,
            download_and_extract_alist,
            get_alist_version,
            set_alist_password
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}