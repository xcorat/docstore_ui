mod constants;

pub use constants::*;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::RwLock;
use crate::db::DbConnection;

#[derive(Default)]
pub struct AppState {
    root_path: RwLock<Option<PathBuf>>,
    db: RwLock<Option<DbConnection>>,
}

impl AppState {
    pub fn new() -> Self {
        // Create the directory if it doesn't exist
        let default_path = default_root_path();
        if !default_path.exists() {
            std::fs::create_dir_all(&default_path).unwrap_or_else(|e| {
                eprintln!("Failed to create default directory: {}", e);
            });
        }

        // Initialize database connection
        let db_path = default_path.join("docstore.db");
        let db = DbConnection::new(db_path).unwrap_or_else(|e| {
            eprintln!("Failed to create database connection: {}", e);
            panic!("Database connection is required for the application to function");
        });

        AppState {
            root_path: RwLock::new(Some(default_path)),
            db: RwLock::new(Some(db)),
        }
    }

    pub fn get_root_path(&self) -> Option<PathBuf> {
        self.root_path.read().ok()?.clone()
    }

    pub fn get_db(&self) -> Option<std::sync::RwLockReadGuard<'_, Option<DbConnection>>> {
        self.db.read().ok()
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
