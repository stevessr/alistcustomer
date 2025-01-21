use std::process::Command;
//use std::str;
//use regex::Regex;
use std::path::PathBuf;
use tauri::State;
use crate::alist::share::AlistPath;
//use serde::Serialize;

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

// 初始化 AlistPath 结构体
pub async fn init_alist_path(alist_path: State<'_, AlistPath>) -> Result<(), String> {
    if let Some(path) = find_alist() {
        let path_str = path.to_string_lossy().into_owned();
        alist_path.with_path(|p| *p = Some(path_str)).await?;
        Ok(())
    } else {
        Err("Failed to find alist executable".to_string())
    }
}
