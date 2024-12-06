#[cfg(test)]
mod tests {
    use super::super::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;
    use rocket::http::ContentType;
    use std::fs;
    use tempfile::TempDir;

    fn setup_client() -> (Client, TempDir) {
        // Create a temporary directory for testing
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        
        // Create a test file in the temp directory
        let test_file_path = temp_dir.path().join("test.txt");
        fs::write(&test_file_path, "test content").expect("Failed to write test file");
        
        let client = Client::tracked(rocket()).expect("Failed to create client");
        (client, temp_dir)
    }

    #[test]
    fn test_index() {
        let (client, _temp_dir) = setup_client();
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        
        let response_json: serde_json::Value = serde_json::from_str(
            &response.into_string().unwrap()
        ).unwrap();
        
        assert_eq!(response_json["status"], "success");
        assert_eq!(response_json["message"], "DocStore API is running");
    }

    #[test]
    fn test_get_root_path() {
        let (client, _temp_dir) = setup_client();
        let response = client.get("/config/path").dispatch();
        assert_eq!(response.status(), Status::Ok);
        
        let response_json: serde_json::Value = serde_json::from_str(
            &response.into_string().unwrap()
        ).unwrap();
        
        assert_eq!(response_json["status"], "success");
        // The message should contain a path
        assert!(response_json["message"].as_str().unwrap().len() > 0);
    }

    #[test]
    fn test_set_root_path() {
        let (client, temp_dir) = setup_client();
        let test_path = temp_dir.path().to_string_lossy().to_string();
        
        // Ensure the directory exists
        std::fs::create_dir_all(&test_path).expect("Failed to create test directory");
        
        let response = client.post("/config/path")
            .header(ContentType::JSON)
            .json(&serde_json::json!({
                "path": test_path
            }))
            .dispatch();
        
        assert_eq!(response.status(), Status::Ok);
        
        let response_json: serde_json::Value = serde_json::from_str(
            &response.into_string().unwrap()
        ).unwrap();
        
        assert_eq!(response_json["status"], "success");
        assert!(response_json["message"].as_str().unwrap().contains(&test_path));
        
        // Verify the path was actually set by making a GET request
        let get_response = client.get("/config/path").dispatch();
        assert_eq!(get_response.status(), Status::Ok);
        
        let get_json: serde_json::Value = serde_json::from_str(
            &get_response.into_string().unwrap()
        ).unwrap();
        
        assert_eq!(get_json["status"], "success");
        assert!(get_json["message"].as_str().unwrap().contains(&test_path));
    }

    #[test]
    fn test_set_invalid_root_path() {
        let (client, _temp_dir) = setup_client();
        let response = client.post("/config/path")
            .json(&serde_json::json!({
                "path": "/non/existent/path"
            }))
            .dispatch();
        
        assert_eq!(response.status(), Status::Ok);
        
        let response_json: serde_json::Value = serde_json::from_str(
            &response.into_string().unwrap()
        ).unwrap();
        
        assert_eq!(response_json["status"], "error");
        assert!(response_json["message"].as_str().unwrap().contains("Invalid path"));
    }


    #[test]
    fn test_get_file() {
        let (client, temp_dir) = setup_client();
        
        // First set the root path
        let test_path = temp_dir.path().to_string_lossy().to_string();
        client.post("/config/path")
            .header(ContentType::JSON)
            .json(&serde_json::json!({
                "path": test_path
            }))
            .dispatch();
        
        // Create a test file
        let test_file = "test.txt";
        let test_content = "test content";
        fs::write(temp_dir.path().join(test_file), test_content).expect("Failed to write test file");
        
        // Try to get the file
        let response = client.get(format!("/files/{}", test_file)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), test_content);
    }

    #[test]
    fn test_get_nonexistent_file() {
        let (client, temp_dir) = setup_client();
        
        // First set the root path
        let test_path = temp_dir.path().to_string_lossy().to_string();
        client.post("/config/path")
            .header(ContentType::JSON)
            .json(&serde_json::json!({
                "path": test_path
            }))
            .dispatch();
        
        // Try to get a non-existent file
        let response = client.get("/files/nonexistent.txt").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn test_file_upload() {
        let (client, temp_dir) = setup_client();
        
        // First set the root path
        let test_path = temp_dir.path().to_string_lossy().to_string();
        client.post("/config/path")
            .header(ContentType::JSON)
            .json(&serde_json::json!({
                "path": test_path
            }))
            .dispatch();
        
        // Create a test file
        let test_file = "test.txt";
        let test_content = "test content";
        let file_path = temp_dir.path().join(test_file);
        fs::write(&file_path, test_content).expect("Failed to write test file");
        
        // Create form data
        let boundary = "test_boundary";
        let content_type = format!("multipart/form-data; boundary={}", boundary);
        let body = format!(
            "--{}\r\nContent-Disposition: form-data; name=\"files\"; filename=\"{}\"\r\n\r\n{}\r\n--{}--\r\n",
            boundary, test_file, test_content, boundary
        );
        
        let response = client.post("/files/upload/test_client")
            .header(ContentType::parse_flexible(&content_type).unwrap())
            .body(body)
            .dispatch();
            
        assert_eq!(response.status(), Status::Ok);
        
        let response_json: serde_json::Value = serde_json::from_str(
            &response.into_string().unwrap()
        ).unwrap();
        
        let files = response_json["files"].as_array().unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0], test_file);
    }

    #[test]
    fn test_file_upload_multiple_files() {
        let (client, _temp_dir) = setup_client();
        
        // Create a test file content
        let boundary = "------------------------14737809831466499882746641449";
        let content = format!(
            "--{boundary}\r\n\
             Content-Disposition: form-data; name=\"files\"; filename=\"test1.txt\"\r\n\
             Content-Type: text/plain\r\n\r\n\
             Hello, World!\r\n\
             --{boundary}\r\n\
             Content-Disposition: form-data; name=\"files\"; filename=\"test2.txt\"\r\n\
             Content-Type: text/plain\r\n\r\n\
             Another file\r\n\
             --{boundary}--\r\n",
            boundary = boundary
        );

        // Create the request with proper headers
        let response = client
            .post("/files/upload/test-client")
            .header(ContentType::parse_flexible(&format!("multipart/form-data; boundary={}", boundary)).unwrap())
            .body(content)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        
        let response_json: serde_json::Value = serde_json::from_str(
            &response.into_string().unwrap()
        ).unwrap();
        
        let files = response_json["files"].as_array().unwrap();
        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|f| f == "test1.txt"));
        assert!(files.iter().any(|f| f == "test2.txt"));
    }

    #[test]
    fn test_file_upload_no_files() {
        let (client, _temp_dir) = setup_client();
        
        // Create an empty form
        let boundary = "------------------------14737809831466499882746641449";
        let content = format!(
            "--{boundary}\r\n\
             --{boundary}--\r\n",
            boundary = boundary
        );

        let response = client
            .post("/files/upload/test-client")
            .header(ContentType::parse_flexible(&format!("multipart/form-data; boundary={}", boundary)).unwrap())
            .body(content)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        
        let response_json: serde_json::Value = serde_json::from_str(
            &response.into_string().unwrap()
        ).unwrap();
        
        let files = response_json["files"].as_array().unwrap();
        assert_eq!(files.len(), 0);
    }
}
