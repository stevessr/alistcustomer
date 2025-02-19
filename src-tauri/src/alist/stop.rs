use crate::alist::share::{AlistState, AlistStatus};
// 停止 alist
#[tauri::command]
pub async fn stop_alist(state: tauri::State<'_, AlistState>) -> Result<AlistStatus, String> {
    state.with_process(|alist_process| {
        if let Some(mut process) = alist_process.take() {
            match process.kill() {
                Ok(_) => Ok(AlistStatus::new(false, None)),
                Err(e) => Err(format!("Failed to kill process: {}", e))
            }
        } else {
            Ok(AlistStatus::new(false, None))
        }
    }).await?
}
