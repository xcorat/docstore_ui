#[macro_use] extern crate rocket;

mod config;
#[cfg(test)]
mod tests;

use rocket::serde::json::Json;
use crate::config::{AppState, ApiResponse};
use std::path::PathBuf;
use rocket::fs::NamedFile;
use rocket::State;
use rocket::http::ContentType;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataError, MultipartFormDataField, MultipartFormDataOptions, Repetition
};
use rocket::Data;
use uuid::Uuid;
use std::fs;

#[get("/")]
async fn index() -> Json<ApiResponse> {
    Json(ApiResponse {
        status: "success".to_string(),
        message: "DocStore API is running".to_string(),
    })
}

#[get("/files/<path..>")]
async fn get_file(path: PathBuf, state: &State<AppState>) -> Option<NamedFile> {
    println!("Request path: {:?}", path);
    let root_path = state.get_root_path().unwrap();
    NamedFile::open(root_path.join(path)).await.ok()
}

#[post("/files/upload/<client_id>", data = "<data>")]
async fn upload_files(content_type: &ContentType, data: Data<'_>, client_id: String, state: &State<AppState>) -> Json<ApiResponse> {
    println!("Request path: /files/upload/{}", client_id);
    println!("Content type: {:?}", content_type);
    // println!("Data length: {}", );
    let root_path = match state.get_root_path() {
        Some(path) => path,
        None => return Json(ApiResponse {
            status: "error".to_string(),
            message: "Root path not configured".to_string(),
        }),
    };

    // Create client directory if it doesn't exist
    let client_dir = root_path.join(&client_id);
    if !client_dir.exists() {
        if let Err(e) = fs::create_dir_all(&client_dir) {
            return Json(ApiResponse {
                status: "error".to_string(),
                message: format!("Failed to create client directory: {}", e),
            });
        }
    }

    // Configure multipart form options
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::file("files")
            .size_limit(10 * 1024 * 1024) // 10MB size limit
            .repetition(Repetition::infinite()), // Allow multiple files
    ]);

    // Process the multipart form data
    let multipart_form_data = match MultipartFormData::parse(content_type, data, options).await {
        Ok(multipart) => multipart,
        Err(e) => return Json(ApiResponse {
            status: "error".to_string(),
            message: format!("Failed to parse multipart form data: {}", e),
        }),
    };
    
    let files = multipart_form_data.files.get("files");
    let mut saved_files = Vec::new();

    if let Some(files) = files {
        for file in files {
            let file_name = match &file.file_name {
                Some(name) => name.clone(),
                None => Uuid::new_v4().to_string(),
            };
            
            let file_path = client_dir.join(&file_name);
            if let Err(e) = fs::copy(&file.path, &file_path) {
                return Json(ApiResponse {
                    status: "error".to_string(),
                    message: format!("Failed to save file {}: {}", file_name, e),
                });
            }
            saved_files.push(file_name);
        }
    }

    Json(ApiResponse {
        status: "success".to_string(),
        message: format!("Successfully uploaded {} files", saved_files.len()),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(AppState::new())
        .mount("/", routes![index, get_file, upload_files])
        .mount("/config", routes![config::get_root_path, config::set_root_path])
}
