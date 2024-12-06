use rocket::serde::json::Json;
use rocket::State;
use rocket::fs::NamedFile;
use rocket::{get, post};
use rocket::http::ContentType;
use rocket::Data;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions, Repetition
};
use serde::Serialize;
use std::path::PathBuf;
use std::fs;
use uuid::Uuid;
use crate::config::{AppState, ApiResponse};

#[derive(Serialize)]
pub struct FileList {
    files: Vec<String>
}

#[get("/")]
pub async fn index() -> Json<ApiResponse> {
    Json(ApiResponse {
        status: "success".to_string(),
        message: "DocStore API is running".to_string(),
    })
}

#[get("/files/<path..>")]
pub async fn get_file(path: PathBuf, state: &State<AppState>) -> Option<NamedFile> {
    println!("Request path: {:?}", path);
    let root_path = state.get_root_path().unwrap();
    NamedFile::open(root_path.join(path)).await.ok()
}

#[post("/files/upload/<client_id>", data = "<data>")]
pub async fn upload_files(content_type: &ContentType, data: Data<'_>, client_id: String, state: &State<AppState>) -> Json<FileList> {
    println!("Request path: /files/upload/{}", client_id);
    println!("Content type: {:?}", content_type);
    let root_path = match state.get_root_path() {
        Some(path) => path,
        None => return Json(FileList { files: vec![] }),
    };

    // Create client directory if it doesn't exist
    let client_dir = root_path.join(&client_id);
    if !client_dir.exists() {
        if let Err(_) = fs::create_dir_all(&client_dir) {
            return Json(FileList { files: vec![] });
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
        Err(_) => return Json(FileList { files: vec![] }),
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
            if let Err(_) = fs::copy(&file.path, &file_path) {
                continue;
            }
            saved_files.push(file_name);
        }
    }

    Json(FileList { files: saved_files })
}
