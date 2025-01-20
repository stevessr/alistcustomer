use std::fs;
use serde::{Deserialize, Serialize};
use crate::alist::find_file::find_alist;


#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub force: bool,
    pub site_url: String,
    pub cdn: String,
    pub jwt_secret: String,
    pub token_expires_in: i32,
    pub database: Database,
    pub meilisearch: Meilisearch,
    pub scheme: Scheme,
    pub temp_dir: String,
    pub bleve_dir: String,
    pub dist_dir: String,
    pub log: Log,
    pub delayed_start: i32,
    pub max_connections: i32,
    pub tls_insecure_skip_verify: bool,
    pub tasks: Tasks,
    pub cors: Cors,
    pub s3: S3,
    pub ftp: Ftp,
    pub sftp: Sftp,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    #[serde(rename = "type")]
    pub type_: String,
    pub host: String,
    pub port: i32,
    pub user: String,
    pub password: String,
    pub name: String,
    pub db_file: String,
    pub table_prefix: String,
    pub ssl_mode: String,
    pub dsn: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meilisearch {
    pub host: String,
    pub api_key: String,
    pub index_prefix: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Scheme {
    pub address: String,
    pub http_port: i32,
    pub https_port: i32,
    pub force_https: bool,
    pub cert_file: String,
    pub key_file: String,
    pub unix_file: String,
    pub unix_file_perm: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    pub enable: bool,
    pub name: String,
    pub max_size: i32,
    pub max_backups: i32,
    pub max_age: i32,
    pub compress: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tasks {
    pub download: Task,
    pub transfer: Task,
    pub upload: Task,
    pub copy: Task,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub workers: i32,
    pub max_retry: i32,
    pub task_persistant: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cors {
    pub allow_origins: Vec<String>,
    pub allow_methods: Vec<String>,
    pub allow_headers: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct S3 {
    pub enable: bool,
    pub port: i32,
    pub ssl: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ftp {
    pub enable: bool,
    pub listen: String,
    pub find_pasv_port_attempts: i32,
    pub active_transfer_port_non_20: bool,
    pub idle_timeout: i32,
    pub connection_timeout: i32,
    pub disable_active_mode: bool,
    pub default_transfer_binary: bool,
    pub enable_active_conn_ip_check: bool,
    pub enable_pasv_conn_ip_check: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sftp {
    pub enable: bool,
    pub listen: String,
}

// 读取 config.json 文件
#[tauri::command]
pub async fn read_config() -> Result<Config, String> {
    let alist_path = find_alist().ok_or("Failed to find alist executable".to_string())?;
    let config_dir = alist_path.parent().ok_or("Failed to get parent directory".to_string())?;
    let config_path = config_dir.join("data/config.json");
    let data = fs::read_to_string(config_path).map_err(|e| e.to_string())?;
    serde_json::from_str(&data).map_err(|e| e.to_string())
}

// 写入 config.json 文件
#[tauri::command]
pub async fn write_config(config: Config) -> Result<(), String> {
    let alist_path = find_alist().ok_or("Failed to find alist executable".to_string())?;
    let config_dir = alist_path.parent().ok_or("Failed to get parent directory".to_string())?;
    let config_path = config_dir.join("data/config.json");
    let data = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    fs::write(config_path, data).map_err(|e| e.to_string())
}