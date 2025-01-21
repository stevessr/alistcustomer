//use std::process::Child;
use tokio::sync::Mutex;
use log::{error, info};
use crate::alist::share::{AlistState, AlistStatus, AlistPath};
use crate::alist::find_process::find_existing_alist_process;

#[tauri::command]
pub async fn get_alist_status(
    state: tauri::State<'_, Mutex<AlistState>>,
    alist_path: tauri::State<'_, Mutex<AlistPath>>
) -> Result<AlistStatus, String> {
    check_alist_status(&state, &alist_path).await
}

async fn check_alist_status(
    state: &tauri::State<'_, Mutex<AlistState>>,
    alist_path: &tauri::State<'_, Mutex<AlistPath>>
) -> Result<AlistStatus, String> {
    // Check managed process status
    let managed_status = check_managed_process(state).await?;
    if let Some(status) = managed_status {
        info!("Found running managed process with PID: {:?}", status.pid);
        return Ok(status);
    }

    // Check for external process if no managed process found
    let external_status = check_external_process(alist_path).await?;
    if let Some(status) = external_status {
        info!("Found external process with PID: {:?}", status.pid);
        return Ok(status);
    }

    info!("No running AList processes found");
    Ok(AlistStatus {
        running: false,
        pid: None,
    })
}

async fn check_managed_process(
    state: &tauri::State<'_, Mutex<AlistState>>
) -> Result<Option<AlistStatus>, String> {
    let state = state.lock().await;
    let mut process = state.0.lock().await;
    
    if let Some(process) = &mut *process {
        match process.try_wait() {
            Ok(None) => Ok(Some(AlistStatus {
                running: true,
                pid: Some(process.id()),
            })),
            Ok(Some(_)) => Ok(None),
            Err(e) => {
                error!("Failed to check managed process status: {}", e);
                Err(format!("Failed to check managed process status: {}", e))
            }
        }
    } else {
        Ok(None)
    }
}

async fn check_external_process(
    alist_path: &tauri::State<'_, Mutex<AlistPath>>
) -> Result<Option<AlistStatus>, String> {
    let guard = alist_path.lock().await;
    // 直接访问内部的 Option<String>
    let path = guard.0.lock().await;
    
    // 现在可以直接使用 as_ref() 因为现在操作的是 Option<String>
    if let Some(path_str) = path.as_ref() {
        if let Some(pid) = find_existing_alist_process(path_str) {
            return Ok(Some(AlistStatus {
                running: true,
                pid: Some(pid),
            }));
        }
    }
    
    Ok(None)
}