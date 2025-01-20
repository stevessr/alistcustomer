// 停止 alist
#[tauri::command]
pub async fn stop_alist(state: tauri::State<'_, AlistState>) -> Result<AlistStatus, String> {
    let mut alist_process = state.0.lock().unwrap();
    
    if let Some(mut process) = alist_process.take() {
        match process.kill() {
            Ok(_) => Ok(AlistStatus {
                running: false,
                pid: None,
            }),
            Err(e) => Err(format!("Failed to stop alist: {}", e))
        }
    } else {
        Ok(AlistStatus {
            running: false,
            pid: None,
        })
    }
}