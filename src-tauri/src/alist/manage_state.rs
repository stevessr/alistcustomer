use tauri::command;
use sysinfo::System;
use crate::alist::share::ProcessMetrics;

#[command]
pub fn manage_alist_state() -> Result<ProcessMetrics, String> {
    let mut system = System::new_all();
    system.refresh_all();

    // Get the AList process information
    if let Some((_, process)) = system.processes().iter().find(|(_, p)| p.name() == "alist") {
        Ok(ProcessMetrics {
            pid: Some((process.pid().as_u32() as i32).to_string()),
            running: true,
            cpu_usage: Some(process.cpu_usage()),
            memory_usage: Some(process.memory()),
        })
    } else {
        Err("AList process not found".to_string())
    }
}

#[command]
pub async fn get_alist_status() -> Result<ProcessMetrics, String> {
    let mut system = System::new_all();
    system.refresh_processes(); // 更高效的进程刷新
    
    // 改进的进程查找逻辑
    let process = system.processes()
        .values()
        .find(|p| {
            let name = p.name().to_lowercase();
            name == "alist" || name == "alist.exe" // 兼容不同平台的进程名
        });
    
    match process {
        Some(p) => Ok(ProcessMetrics {
            pid: Some(p.pid().as_u32().to_string()), // 简化类型转换
            running: true,
            cpu_usage: Some(p.cpu_usage()),
            memory_usage: Some(p.memory()),
        }),
        None => Ok(ProcessMetrics {
            pid: None,
            running: false,
            cpu_usage: None,
            memory_usage: None,
        })
    }
}

#[command]
pub async fn get_metrics() -> Result<ProcessMetrics, String> {
    get_alist_status().await
}
