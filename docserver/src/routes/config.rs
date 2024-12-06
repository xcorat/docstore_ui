
use rocket::{get, post};
use rocket::serde::json::Json;
use rocket::State;
use serde::Deserialize;
use std::path::PathBuf;

use crate::config::{AppState, ApiResponse};

#[derive(Deserialize)]
pub struct SetRootPathRequest {
    path: String,
}

#[get("/path")]
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

#[post("/path", format = "json", data = "<request>")]
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
    state.set_root_path(absolute_path.clone());
    Json(ApiResponse {
        status: "success".to_string(),
        message: format!("Root path set to: {}", absolute_path.to_string_lossy()),
    })
}
