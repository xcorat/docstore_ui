use std::path::PathBuf;

// Database settings
pub const DEFAULT_DB_FILENAME: &str = "docstore.db";

// File storage settings
pub fn default_root_path() -> PathBuf {
    std::env::temp_dir().join("docstore_files")
}
