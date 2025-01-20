use std::process::Command;
use std::sync::Mutex;
use tauri::State;

// 定义一个结构体来存储 alist 的路径
pub struct AlistPath(pub Mutex<Option<String>>);

#[tauri::command]
pub async fn set_alist_password(
    new_password: String,
    alist_path: State<'_, AlistPath>,
) -> Result<(), String> {
    // 获取 alist 的路径
    let alist_path = alist_path.0.lock().unwrap().clone().ok_or("Alist path not set".to_string())?;

    // 构建命令
    let output = Command::new(&alist_path)
        .arg("admin")
        .arg("set")
        .arg(&new_password)
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    // 检查命令是否成功
    if output.status.success() {
        Ok(())
    } else {
        let error_message = String::from_utf8(output.stderr).unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("Failed to set password: {}", error_message))
    }
}
