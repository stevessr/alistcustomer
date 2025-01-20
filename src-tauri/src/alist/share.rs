use std::process::Child;
use std::sync::Mutex;
use serde::Serialize;


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
pub struct AlistState(pub Mutex<Option<Child>>);

impl AlistState {
    /// Safely access the process state
    pub fn with_process<F, R>(&self, f: F) -> Result<R, String>
    where
        F: FnOnce(&mut Option<Child>) -> R,
    {
        self.0.lock()
            .map_err(|e| format!("Failed to lock process state: {}", e))
            .map(|mut guard| f(&mut *guard))
    }
}

/// Wrapper for managing the alist executable path
pub struct AlistPath(pub std::sync::Mutex<Option<String>>);

impl AlistPath {
    /// Safely access the path
    pub fn with_path<F, R>(&self, f: F) -> Result<R, String>
    where
        F: FnOnce(&mut Option<String>) -> R,
    {
        self.0.lock()
            .map_err(|e| format!("Failed to lock path state: {}", e))
            .map(|mut guard| f(&mut *guard))
    }
}
