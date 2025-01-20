use std::fs;
//use std::path::PathBuf;
//use std::process::{Command, Child};
//use std::sync::Mutex;
//use serde::Serialize;
#[allow(unused_imports)]
use sysinfo::{ProcessExt, System, SystemExt, Pid, PidExt};  // 导入 PidExt
//use std::path::Path;
use crate::alist::find_file::find_alist;


// 删除 alist 所在目录下的 data 文件夹
#[tauri::command]
pub async fn delete_data_folder() -> Result<(), String> {
    let alist_path = find_alist().ok_or_else(|| "Failed to find alist executable".to_string())?;
    let alist_dir = alist_path.parent().ok_or_else(|| "Failed to get alist directory".to_string())?;
    let data_dir = alist_dir.join("data");

    if data_dir.exists() {
        fs::remove_dir_all(&data_dir).map_err(|e| format!("Failed to delete data folder: {}", e))
    } else {
        Err("Data folder does not exist".to_string())
    }
}