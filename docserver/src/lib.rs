pub mod db;
pub mod config;
pub mod routes;

// Re-export key types to make them easily accessible
pub use db::{Database, Client, TaxReturn};
