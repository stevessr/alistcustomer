use std::env::consts::{OS, ARCH};
use std::io::{Cursor, Read};
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::task::spawn_blocking;
use zip::ZipArchive;

#[tauri::command]
pub async fn download_and_extract_alist(proxy_url: Option<String>) -> Result<(), String> {
    // 根据系统架构和操作系统类型构建下载URL
    let download_url = match (OS, ARCH) {
        ("linux", "x86_64") => "https://github.com/AlistGo/alist/releases/latest/download/alist-linux-amd64.zip",
        ("linux", "aarch64") => "https://github.com/AlistGo/alist/releases/latest/download/alist-linux-arm64.zip",
        ("windows", "x86_64") => "https://github.com/AlistGo/alist/releases/latest/download/alist-windows-amd64.zip",
        ("macos", "x86_64") => "https://github.com/AlistGo/alist/releases/latest/download/alist-darwin-amd64.zip",
        ("macos", "aarch64") => "https://github.com/AlistGo/alist/releases/latest/download/alist-darwin-arm64.zip",
        _ => return Err(format!("Unsupported OS/Architecture: {}/{}", OS, ARCH)),
    };

    // 创建 reqwest 客户端
    let client = if let Some(proxy) = proxy_url {
        Client::builder()
            .proxy(reqwest::Proxy::all(&proxy).map_err(|e| format!("Failed to set proxy: {}", e))?)
            .build()
            .map_err(|e| format!("Failed to create client with proxy: {}", e))?
    } else {
        Client::new()
    };

    // 使用 reqwest 异步下载文件
    let response = client.get(download_url)
        .send()
        .await
        .map_err(|e| format!("Failed to download alist: {}", e))?;

    let bytes = response.bytes()
        .await
        .map_err(|e| format!("Failed to read response bytes: {}", e))?;

    // 使用 zip 库解压文件
    let mut archive = ZipArchive::new(Cursor::new(bytes.clone()))
        .map_err(|e| format!("Failed to open zip archive: {}", e))?;

    // 将 bytes 克隆到循环外部
    let value = bytes.clone();

    for i in 0..archive.len() {
        let outpath = archive.by_index(i)
            .map_err(|e| format!("Failed to read file in zip archive: {}", e))?
            .name().to_string();

        // 在每次迭代时克隆 value
        let value_clone = value.clone();

        let contents = spawn_blocking(move || {
            let mut archive = ZipArchive::new(Cursor::new(value_clone))
                .map_err(|e| format!("Failed to open zip archive: {}", e))?;
            let mut file = archive.by_index(i)
                .map_err(|e| format!("Failed to read file in zip archive: {}", e))?;

            let mut contents = Vec::new();
            let mut buffer = [0u8; 1024]; // 使用缓冲区读取文件
            loop {
                let bytes_read = file.read(&mut buffer)
                    .map_err(|e| format!("Failed to read file contents: {}", e))?;
                if bytes_read == 0 {
                    break;
                }
                contents.extend_from_slice(&buffer[..bytes_read]);
            }

            Ok::<Vec<u8>, String>(contents)
        }).await.map_err(|e| format!("Failed to spawn blocking task: {}", e))??;

        let mut outfile = File::create(&outpath)
            .await
            .map_err(|e| format!("Failed to create file: {}", e))?;

        outfile.write_all(&contents)
            .await
            .map_err(|e| format!("Failed to write file: {}", e))?;
    }

    Ok(())
}
