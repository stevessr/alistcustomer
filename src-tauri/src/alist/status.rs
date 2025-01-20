#[allow(unused_imports)]
use std::fs;
#[allow(unused_imports)]
use std::path::PathBuf;
#[allow(unused_imports)]
use std::process::{Command, Child};
#[allow(unused_imports)]
use std::sync::Mutex;
#[allow(unused_imports)]
use serde::Serialize;
#[allow(unused_imports)]
use sysinfo::{ProcessExt, System, SystemExt, Pid, PidExt};  // 导入 PidExt
#[allow(unused_imports)]
use std::path::Path;
use crate::alist::share::{AlistState, AlistStatus};
use crate::alist::find_process::find_existing_alist_process;

// 获取 alist 状态
#[tauri::command]
pub async fn get_alist_status(state: tauri::State<'_, AlistState>) -> Result<AlistStatus, String> {
    let mut alist_process = state.0.lock().unwrap();

    // 优先检查系统中是否已经存在 alist 进程
    if let Some(existing_pid) = find_existing_alist_process(&Path::new("/path/to/alist")) {
        // 如果找到已存在的进程，更新 AlistState 中的信息
        *alist_process = Some(Command::new("/path/to/alist").spawn().map_err(|e| {
            format!("Failed to spawn alist process: {}", e)
        })?);
        return Ok(AlistStatus {
            running: true,
            pid: Some(existing_pid),
        });
    }
    

    // 如果没有找到已存在的进程，检查 AlistState 中的进程
    if let Some(process) = &mut *alist_process {
        match process.try_wait() {
            Ok(None) => Ok(AlistStatus {
                running: true,
                pid: Some(process.id()),
            }),
            Ok(Some(_)) => Ok(AlistStatus {
                running: false,
                pid: None,
            }),
            Err(e) => Err(format!("Failed to check alist status: {}", e)),
        }
    } else {
        Ok(AlistStatus {
            running: false,
            pid: None,
        })
    }
}
