use std::process::Command;
use sysinfo::{ProcessExt, System, SystemExt, Pid, PidExt};  // 导入 PidExt

// 启动 alist
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

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

    // 检查文件权限
    let metadata = fs::metadata(&alist_path).map_err(|e| format!("Failed to get metadata: {}", e))?;
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
            *alist_process = Some(child);
            Ok(AlistStatus {
                running: true,
                pid: Some(pid),
            })
        }
        Err(e) => Err(format!("Failed to start alist: {}", e))
    }
}