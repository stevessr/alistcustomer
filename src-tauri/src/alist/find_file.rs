use std::process::Command;
use std::str;
use regex::Regex;
use std::path::PathBuf;
use serde::Serialize;

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