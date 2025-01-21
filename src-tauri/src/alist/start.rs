use std::process::Command;
#[allow(unused_imports)]
use std::fs::{self, Permissions};
#[allow(unused_imports)]
use sysinfo::{ProcessExt, System, SystemExt, Pid, PidExt};  // 导入 PidExt
use crate::alist::share::{AlistState, AlistStatus};
use crate::alist::find_file::find_alist;
use crate::alist::find_process::find_existing_alist_process;

// 启动 alist
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

impl AlistState {
    // 检查 alist 是否已经在运行
    pub async fn is_alist_running(&self) -> bool {
        self.with_process(|alist_process| {
            alist_process.as_mut().map_or(false, |p: &mut std::process::Child| p.try_wait().is_ok())
        }).await.unwrap_or(false) // 处理 Result<bool, String>，返回 bool
    }

    // 处理 alist 进程的退出
    pub async fn handle_process_exit(&self) {
        self.with_process(|alist_process| {
            if let Some(process) = alist_process {
                if process.try_wait().is_ok() {
                    *alist_process = None;
                }
            }
        }).await.unwrap_or(()); // 忽略错误
    }
}

#[tauri::command]
pub async fn start_alist(state: tauri::State<'_, AlistState>) -> Result<AlistStatus, String> {
    // 检查是否已经在运行
    if state.is_alist_running().await {
        let pid = state.with_process(|p| p.as_ref().unwrap().id()).await?; // 使用 ? 解包 Result<u32, String>
        return Ok(AlistStatus::new(true, Some(pid)));
    }

    // 自动寻找 alist 可执行文件
    let alist_path = find_alist().ok_or_else(|| "Failed to find alist executable".to_string())?;
    println!("alist_path: {:?}", alist_path);

    // 检查文件权限
    let metadata = fs::metadata(&alist_path).map_err(|e| format!("Failed to get metadata: {}", e))?;
    #[allow(unused_variables)]
    let permissions = metadata.permissions();

    #[cfg(unix)]
    {
        if permissions.mode() & 0o111 == 0 {
            // 如果没有执行权限，尝试设置权限
            let new_permissions = Permissions::from_mode(0o755); // 设置可执行权限
            fs::set_permissions(&alist_path, new_permissions).map_err(|e| format!("Failed to set permissions: {}", e))?;
        }
    }

    #[cfg(windows)]
    {
        if metadata.permissions().readonly() {
            // 如果文件是只读的，尝试设置为可写
            let mut permissions = metadata.permissions();
            permissions.set_readonly(false);
            fs::set_permissions(&alist_path, permissions).map_err(|e| format!("Failed to set permissions: {}", e))?;
        }
    }

    // 检查系统中是否已经存在 alist 进程
    if let Some(pid) = find_existing_alist_process(&alist_path) {
        return Ok(AlistStatus {
            running: true,
            pid: Some(pid),
        });
    }

    // 启动新的 alist 进程
    match Command::new(alist_path)
        .arg("server")
        .spawn() {
        Ok(child) => {
            let pid = child.id();
            state.with_process(|alist_process| {
                *alist_process = Some(child);
                Ok(AlistStatus::new(true, Some(pid)))
            }).await?
        }
        Err(e) => Err(format!("Failed to start alist: {}", e))
    }
}
