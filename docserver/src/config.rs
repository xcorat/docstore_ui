use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::RwLock;

#[derive(Default)]
pub struct AppState {
    root_path: RwLock<Option<PathBuf>>,
}

impl AppState {
    pub fn new() -> Self {
        // Set default path to a temporary directory
        let default_path = std::env::temp_dir().join("docstore_files");
        // Create the directory if it doesn't exist
        if !default_path.exists() {
            std::fs::create_dir_all(&default_path).unwrap_or_else(|e| {
                eprintln!("Failed to create default directory: {}", e);
            });
        }
        AppState {
            root_path: RwLock::new(Some(default_path)),
        }
    }

    pub fn get_root_path(&self) -> Option<PathBuf> {
        self.root_path.read().ok()?.clone()
    }

    pub fn set_root_path(&self, path: PathBuf) {
        *self.root_path.write().unwrap() = Some(path);
    }
}

#[derive(Serialize)]
pub struct ApiResponse {
    pub status: String,
    pub message: String,
}

#[derive(Deserialize)]
pub struct SetRootPathRequest {
    path: String,
}
