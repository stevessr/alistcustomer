use sysinfo::System;
use std::path::Path;

//寻找已存在的alist进程
pub fn find_existing_alist_process(alist_path: &Path) -> Option<u32> {
    let mut system = System::new_all();
    system.refresh_all();

    for (pid, process) in system.processes() {
        let exe_path = process.exe();  // 直接获取路径
        if exe_path == Some(alist_path) {    // 比较路径
            return Some(pid.as_u32()); // 使用 PidExt 的 as_u32() 方法
        }
    }

    None
}
