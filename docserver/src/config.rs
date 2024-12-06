use rocket::serde::json::Json;
use rocket::State;
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

#[get("/root")]
pub async fn get_root_path(state: &State<AppState>) -> Json<ApiResponse> {
    let root_path = state.get_root_path();
    match root_path {
        Some(path) => Json(ApiResponse {
            status: "success".to_string(),
            message: path.to_string_lossy().to_string(),
        }),
        None => Json(ApiResponse {
            status: "not_set".to_string(),
            message: "Root path has not been set".to_string(),
        }),
    }
}

#[post("/root", format = "json", data = "<request>")]
pub async fn set_root_path(request: Json<SetRootPathRequest>, state: &State<AppState>) -> Json<ApiResponse> {
    let path = PathBuf::from(&request.path);
    println!("Request path: {:?}", path);
    // Verify the path exists and is a directory
    if !path.exists() || !path.is_dir() {
        return Json(ApiResponse {
            status: "error".to_string(),
            message: "Invalid path: directory does not exist".to_string(),
        });
    }

    // Store the absolute path
    let absolute_path = path.canonicalize().unwrap_or(path);
    if let Ok(mut root_path) = state.root_path.write() {
        *root_path = Some(absolute_path.clone());
        Json(ApiResponse {
            status: "success".to_string(),
            message: format!("Root path set to: {}", absolute_path.to_string_lossy()),
        })
    } else {
        Json(ApiResponse {
            status: "error".to_string(),
            message: "Failed to set root path".to_string(),
        })
    }
}
