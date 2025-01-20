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

// 获取 alist 状态
#[tauri::command]
pub async fn get_alist_status(state: tauri::State<'_, AlistState>) -> Result<AlistStatus, String> {
    let mut alist_process = state.0.lock().unwrap();
    
    if let Some(process) = &mut *alist_process {
        // 使用可变引用调用 try_wait
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