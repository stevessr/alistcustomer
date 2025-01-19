use std::process::{Command, Child};
use std::sync::Mutex;
use std::env;
use std::path::PathBuf;
use serde::Serialize;

// 将 Mutex 字段设为 pub
pub struct AlistState(pub Mutex<Option<Child>>);

#[derive(Serialize)]
pub struct AlistStatus {
    running: bool,
    pid: Option<u32>,
}

// 自动寻找 alist 可执行文件
pub fn find_alist() -> Option<PathBuf> {
    let command = if cfg!(windows) {
        "where"
    } else {
        "which"
    };

    let output = Command::new(command)
        .arg("alist")
        .output()
        .ok()?;

    if output.status.success() {
        let output_str = String::from_utf8(output.stdout).ok()?;
        // 只取第一行的路径
        let first_line = output_str.lines().next()?;
        Some(PathBuf::from(first_line.trim()))
    } else {
        None
    }
}

// 启动 alist
#[tauri::command]
pub async fn start_alist(state: tauri::State<'_, AlistState>) -> Result<AlistStatus, String> {
    let mut alist_process = state.0.lock().unwrap();
    
    // 检查是否已经在运行
    if let Some(process) = &mut *alist_process {
        match process.try_wait() {
            Ok(None) => return Ok(AlistStatus {
                running: true,
                pid: Some(process.id()),
            }),
            _ => {}
        }
    }

    // 自动寻找 alist 可执行文件
    let alist_path = find_alist().ok_or_else(|| "Failed to find alist executable".to_string())?;
    println!("alist_path: {:?}", alist_path);

    // 启动新的 alist 进程
    match Command::new(alist_path)
        .arg("server")
        .spawn() {
        Ok(child) => {
            let pid = child.id();
            *alist_process = Some(child);
            Ok(AlistStatus {
                running: true,
                pid: Some(pid),
            })
        }
        Err(e) => Err(format!("Failed to start alist: {}", e))
    }
}

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
