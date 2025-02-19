// lib.rs
pub mod alist;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .manage(alist::AlistState::new())
        .manage(alist::AlistPath(tokio::sync::Mutex::new(None::<String>))) // 管理 AlistPath 状态
        .invoke_handler(tauri::generate_handler![
            greet,
            crate::alist::manage_state::get_alist_status,
            crate::alist::start::start_alist,
            crate::alist::stop::stop_alist,
            crate::alist::clean::delete_data_folder,
            crate::alist::download::download_and_extract_alist,
            crate::alist::get_version::get_alist_version,
            crate::alist::reset_password::set_alist_password,
            crate::alist::edit::read_config,
            crate::alist::edit::write_config,
            crate::alist::manage_state::manage_alist_state, // Add this line
            crate::alist::status::get_alist_metrics,
            crate::alist::manage_state::get_metrics
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
