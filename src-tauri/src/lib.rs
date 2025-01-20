mod alist;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(alist::AlistState(std::sync::Mutex::new(None)))
        .manage(alist::AlistPath(std::sync::Mutex::new(None))) // 管理 AlistPath 状态
        .invoke_handler(tauri::generate_handler![
            greet,
            alist::start_alist,
            alist::stop_alist,
            alist::get_alist_status,
            alist::delete_data_folder,
            alist::download_and_extract_alist,
            alist::get_alist_version,
            alist::set_alist_password

        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}