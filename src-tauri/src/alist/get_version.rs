use std::process::Command;
use std::str;
use regex::Regex;
//use std::path::PathBuf;
use serde::Serialize;
use crate::alist::find_file::find_alist;

#[derive(Serialize)]
pub struct AlistVersionInfo {
    built_at: String,
    go_version: String,
    author: String,
    commit_id: String,
    version: String,
    web_version: String,
}

#[tauri::command]
pub async fn get_alist_version() -> Result<AlistVersionInfo, String> {
    // 自动寻找 alist 可执行文件
    let alist_path = find_alist().ok_or_else(|| "Failed to find alist executable".to_string())?;

    // 执行 alist version 命令
    // 添加详细的调试日志
    println!("Executing alist version command at: {}", alist_path.display());
    
    // 处理路径中的空格
    let output = Command::new(&alist_path)
        .arg("version")
        .output()
        .map_err(|e| format!(
            "Failed to execute alist version at '{}': {}\nCheck if the file exists and has execute permissions.",
            alist_path.display(),
            e
        ))?;

    if !output.status.success() {
        let stderr = str::from_utf8(&output.stderr).unwrap_or("Failed to read stderr");
        return Err(format!(
            "Failed to get alist version: {}\nCommand output: {}",
            output.status,
            stderr
        ));
    }

    // 将输出转换为字符串
    let output_str = str::from_utf8(&output.stdout)
        .map_err(|e| format!(
            "Failed to convert output to string: {}\nRaw output: {:?}",
            e,
            output.stdout
        ))?;

    // 使用正则表达式解析输出
    // 更灵活的正则表达式，处理不同格式和空白
    let re = Regex::new(r"(?xi)
        ^Built\s*At:\s*(?P<built_at>[^\n]+)\s*\n
        Go\s*Version:\s*(?P<go_version>[^\n]+)\s*\n
        Author:\s*(?P<author>[^\n]+)\s*\n
        Commit\s*ID:\s*(?P<commit_id>[^\n]+)\s*\n
        Version:\s*(?P<version>[^\n]+)\s*\n
        WebVersion:\s*(?P<web_version>[^\n]+)")
        .map_err(|e| format!("Failed to compile regex: {}", e))?;

    let caps = re.captures(output_str).ok_or_else(|| {
        format!(
            "Failed to parse alist version output. Actual output:\n{}",
            output_str
        )
    })?;

    // 提取并返回解析后的信息
    Ok(AlistVersionInfo {
        built_at: caps["built_at"].to_string(),
        go_version: caps["go_version"].to_string(),
        author: caps["author"].to_string(),
        commit_id: caps["commit_id"].to_string(),
        version: caps["version"].to_string(),
        web_version: caps["web_version"].to_string(),
    })
}
