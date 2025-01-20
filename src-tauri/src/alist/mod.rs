use std::process::Child;
use std::sync::Mutex;
use serde::Serialize;

mod find_file;
use find_file::find_alist;
mod find_process;
use find_process::find_existing_alist_process;
mod start;
use start::start_alist;
mod stop;
use stop::stop_alist;
mod get_version;
use get_version::get_alist_version;
mod reset_password;
use reset_password::set_alist_password;
mod status;
use status::get_alist_status;
mod clean;
use clean::delete_data_folder;
mod download;
use download::download_and_extract_alist;

#[derive(Serialize)]
pub struct AlistStatus {
    running: bool,
    pid: Option<u32>,
}

// 将 Mutex 字段设为 pub
pub struct AlistState(pub Mutex<Option<Child>>);

// 定义全局状态结构体
pub struct AlistPath(pub std::sync::Mutex<Option<String>>);

