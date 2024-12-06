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
        let response = client.get("/config/root").dispatch();
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
        
        let response = client.post("/config/root")
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
    }

    #[test]
    fn test_set_invalid_root_path() {
        let (client, _temp_dir) = setup_client();
        let response = client.post("/config/root")
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
        client.post("/config/root")
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
        client.post("/config/root")
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
        let (client, _temp_dir) = setup_client();
        
        // Create a test file content
        let boundary = "------------------------14737809831466499882746641449";
        let content = format!(
            "--{boundary}\r\n\
             Content-Disposition: form-data; name=\"files\"; filename=\"test.txt\"\r\n\
             Content-Type: text/plain\r\n\r\n\
             Hello, World!\r\n\
             --{boundary}--\r\n",
            boundary = boundary
        );

        // Create the request with proper headers
        let response = client
            .post("/files/upload/test-client")
            .header(ContentType::parse_flexible("multipart/form-data; boundary=------------------------14737809831466499882746641449").unwrap())
            .body(content)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        
        let response_json: serde_json::Value = serde_json::from_str(
            &response.into_string().unwrap()
        ).unwrap();
        
        assert_eq!(response_json["status"], "success");
        
        // Verify that the file was actually saved
        let files = response_json["files"].as_array().unwrap();
        assert_eq!(files.len(), 1);
        assert!(files[0].as_str().unwrap().contains("test.txt"));
    }

    #[test]
    fn test_file_upload_multiple_files() {
        let (client, _temp_dir) = setup_client();
        
        // Create a test file content with multiple files
        let boundary = "------------------------14737809831466499882746641449";
        let content = format!(
            "--{boundary}\r\n\
             Content-Disposition: form-data; name=\"files\"; filename=\"test1.txt\"\r\n\
             Content-Type: text/plain\r\n\r\n\
             Hello, World 1!\r\n\
             --{boundary}\r\n\
             Content-Disposition: form-data; name=\"files\"; filename=\"test2.txt\"\r\n\
             Content-Type: text/plain\r\n\r\n\
             Hello, World 2!\r\n\
             --{boundary}--\r\n",
            boundary = boundary
        );

        // Create the request with proper headers
        let response = client
            .post("/files/upload/test-client")
            .header(ContentType::parse_flexible("multipart/form-data; boundary=------------------------14737809831466499882746641449").unwrap())
            .body(content)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        
        let response_json: serde_json::Value = serde_json::from_str(
            &response.into_string().unwrap()
        ).unwrap();
        
        assert_eq!(response_json["status"], "success");
        
        // Verify that both files were saved
        let files = response_json["files"].as_array().unwrap();
        assert_eq!(files.len(), 2);
        assert!(files[0].as_str().unwrap().contains("test1.txt"));
        assert!(files[1].as_str().unwrap().contains("test2.txt"));
    }

    #[test]
    fn test_file_upload_no_files() {
        let (client, _temp_dir) = setup_client();
        
        // Create an empty multipart form
        let boundary = "------------------------14737809831466499882746641449";
        let content = format!(
            "--{boundary}--\r\n",
            boundary = boundary
        );

        let response = client
            .post("/files/upload/test-client")
            .header(ContentType::parse_flexible("multipart/form-data; boundary=------------------------14737809831466499882746641449").unwrap())
            .body(content)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        
        let response_json: serde_json::Value = serde_json::from_str(
            &response.into_string().unwrap()
        ).unwrap();
        
        assert_eq!(response_json["status"], "error");
        assert!(response_json["message"].as_str().unwrap().contains("No files"));
    }
}
