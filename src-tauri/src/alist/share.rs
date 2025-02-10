use std::process::Child;
use tokio::sync::Mutex;
use serde::Serialize;

#[derive(Serialize)]
pub struct ProcessMetrics {
    pub pid: Option<String>,
    pub running: bool,
    pub cpu_usage: Option<f32>,
    pub memory_usage: Option<u64>,
}

#[derive(Serialize)]
pub struct AlistStatus {
    pub running: bool,
    pub pid: Option<u32>,
}

impl AlistStatus {
    /// Create a new AlistStatus instance
    pub fn new(running: bool, pid: Option<u32>) -> Self {
        Self { running, pid }
    }
}

/// Wrapper for managing the alist process state
pub struct AlistState {
    pub process: Mutex<Option<Child>>,
    pub metrics: Mutex<ProcessMetrics>,
}

impl AlistState {
    /// Create a new AlistState instance
    pub fn new() -> Self {
        Self {
            process: Mutex::new(None),
            metrics: Mutex::new(ProcessMetrics {
                pid: None,
                running: false,
                cpu_usage: None,
                memory_usage: None,
            }),
        }
    }

    /// Safely access the process state
    pub async fn with_process<F, R>(&self, f: F) -> Result<R, String>
    where
        F: FnOnce(&mut Option<Child>) -> R,
    {
        let mut guard = self.process.lock().await;
        Ok(f(&mut *guard))
    }

    /// Safely access the process metrics
    pub async fn with_metrics<F, R>(&self, f: F) -> Result<R, String>
    where
        F: FnOnce(&mut ProcessMetrics) -> R,
    {
        let mut guard = self.metrics.lock().await;
        Ok(f(&mut *guard))
    }
}

/// Wrapper for managing the alist executable path
pub struct AlistPath(pub Mutex<Option<String>>);

impl AlistPath {
    /// Safely access the path
    pub async fn with_path<F, R>(&self, f: F) -> Result<R, String>
    where
        F: FnOnce(&mut Option<String>) -> R,
    {
        let mut guard = self.0.lock().await;
        Ok(f(&mut *guard))
    }
}
